use crate::model::CommitStats;
use anyhow::{Context, Result};
use chrono::TimeZone;
use git2::{Repository, Sort};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn process_diff(_repo: &Repository, diff: &git2::Diff, config: &crate::config::Config, file_map: &mut std::collections::HashMap<String, usize>, file_paths: &mut Vec<String>) -> Result<(usize, usize, Vec<usize>)> {
    let mut added = 0;
    let mut deleted = 0;
    let mut commit_files = std::collections::HashSet::new();

    diff.foreach(&mut |delta, _float| {
        // Skip binary files
        if delta.flags().contains(git2::DiffFlags::BINARY) {
            return true;
        }

        let old_path = delta.old_file().path().and_then(|p| p.to_str());
        let new_path = delta.new_file().path().and_then(|p| p.to_str());

        for path_opt in [old_path, new_path] {
            if let Some(path) = path_opt {
                if is_excluded(path, &config.exclude) {
                    return true;
                }
                
                let path_string = path.to_string();
                let idx = if let Some(&i) = file_map.get(&path_string) {
                    i
                } else {
                    let i = file_paths.len();
                    file_paths.push(path_string.clone());
                    file_map.insert(path_string, i);
                    i
                };
                commit_files.insert(idx);
            }
        }
        true
    }, None, None, Some(&mut |delta, _hunk, line| {
        // Double check exclusion at line level
        let old_path = delta.old_file().path().and_then(|p| p.to_str());
        let new_path = delta.new_file().path().and_then(|p| p.to_str());
        
        if let Some(path) = new_path.or(old_path) {
            if is_excluded(path, &config.exclude) {
                return true;
            }
        }

        match line.origin() {
            '+' => added += 1,
            '-' => deleted += 1,
            _ => {}
        }
        true
    }))?;

    Ok((added, deleted, commit_files.into_iter().collect()))
}

fn is_sync_merge(message: &str, base_branches: &[String]) -> bool {
    let msg_lower = message.to_lowercase();
    
    // Always exclude remote-tracking branch merges as they are typically sync noise
    if msg_lower.contains("merge remote-tracking branch") || msg_lower.contains("merge branch 'origin'") {
        return true;
    }

    for branch in base_branches {
        let b = branch.to_lowercase();
        // Check for standard merge, GitHub PR merge, and "Sync branch" patterns
        if msg_lower.contains(&format!("merge branch '{}'", b)) ||
           msg_lower.contains(&format!("merge remote-tracking branch 'origin/{}'", b)) ||
           (msg_lower.contains("merge pull request") && msg_lower.contains(&format!("from {}", b))) ||
           msg_lower.contains(&format!("merge branch '{}' into", b)) ||
           msg_lower.contains(&format!("sync branch '{}'", b)) ||
           msg_lower.contains(&format!("sync branch with '{}'", b))
        {
            return true;
        }
    }
    false
}

pub fn collect_stats(repo_path: &Path, output_path: &Path, config: &crate::config::Config, merges_only: bool, include_github: bool, no_cache: bool) -> Result<()> {
    let repo = Repository::open(repo_path).context("Failed to open repository")?;
    
    let mut github_prs = Vec::new();
    if include_github {
        let cache_dir = output_path.parent().unwrap_or_else(|| Path::new("."));
        let client = crate::github::GitHubClient::new(repo_path, cache_dir)?;
        
        let mut loaded = false;
        if !no_cache {
            if let Some(cached_prs) = client.load_cache() {
                github_prs = cached_prs;
                loaded = true;
            }
        }
        
        if !loaded {
            github_prs = client.fetch_reviews()?;
            client.save_cache(&github_prs)?;
        }
    }

    // First pass: Count total commits for progress bar
    println!("⏳ Counting commits...");
    let mut revwalk = repo.revwalk()?;
    revwalk.set_sorting(Sort::TIME)?;
    revwalk.push_head()?;
    
    let total_commits = if merges_only {
        let mut count = 0;
        for oid_result in revwalk {
            if let Ok(oid) = oid_result {
                if let Ok(commit) = repo.find_commit(oid) {
                    if commit.parent_count() > 1 {
                        count += 1;
                    }
                }
            }
        }
        count
    } else {
        revwalk.count() as u64
    };

    // Second pass: Process commits
    let mut revwalk = repo.revwalk()?;
    revwalk.set_sorting(Sort::TIME)?;
    revwalk.push_head()?;

    let pb = ProgressBar::new(total_commits);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    let mut stats_list = Vec::new();
    let mut file_paths = Vec::new();
    let mut file_map: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    for oid_result in revwalk {
        let oid = oid_result?;
        let commit = repo.find_commit(oid)?;

        // Check if it's a merge commit
        let is_merge = commit.parent_count() > 1;

        if merges_only && !is_merge {
            continue;
        }

        pb.inc(1);

        let author = commit.author();
        let author_name = author.name().unwrap_or("Unknown").to_string();
        let author_email = author.email().unwrap_or("").to_string();
        
        let time = commit.time();
        let offset = chrono::FixedOffset::east_opt(time.offset_minutes() * 60).unwrap();
        let date = offset.timestamp_opt(time.seconds(), 0).unwrap();

        let commit_message_raw = commit.message().unwrap_or("");
        let commit_message = commit_message_raw.lines().next().unwrap_or("").to_string();
        
        // Skip merges from base branches (sync merges)
        if is_merge && is_sync_merge(commit_message_raw, &config.base_branches) {
            continue;
        }

        let mut lead_time_days = None;
        if is_merge && commit.parent_count() >= 2 {
            let mut revwalk = repo.revwalk()?;
            revwalk.push(commit.parent_id(1)?)?;
            revwalk.hide(commit.parent_id(0)?)?;
            
            let mut oldest_timestamp = commit.time().seconds();
            let mut count = 0;
            for oid_res in revwalk {
                if let Ok(oid) = oid_res {
                    if let Ok(c) = repo.find_commit(oid) {
                        let t = c.time().seconds();
                        if t < oldest_timestamp {
                            oldest_timestamp = t;
                        }
                    }
                }
                count += 1;
                if count > 2000 { break; } // Prevent infinite or too long traversals
            }
            
            let diff_sec = commit.time().seconds() - oldest_timestamp;
            let days = (diff_sec / (24 * 3600)) as u32;
            lead_time_days = Some(days.max(1));
        }

        let (added, deleted, commit_files) = if commit.parent_count() == 0 {
            // Initial commit
            if let Ok(tree) = commit.tree() {
                let diff = repo.diff_tree_to_tree(None, Some(&tree), None)?;
                process_diff(&repo, &diff, config, &mut file_map, &mut file_paths)?
            } else {
                (0, 0, Vec::new())
            }
        } else {
            // Normal or Merge commit (compare with first parent)
            let parent = commit.parent(0)?;
            let tree = commit.tree()?;
            let parent_tree = parent.tree()?;
            let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), None)?;
            process_diff(&repo, &diff, config, &mut file_map, &mut file_paths)?
        };

        stats_list.push(CommitStats {
            hash: oid.to_string(),
            author: author_name,
            date,
            added,
            deleted,
            email: author_email,
            is_merge,
            message: commit_message,
            files: commit_files,
            lead_time_days,
        });
    }

    pb.finish_with_message("Done");

    let report_data = crate::model::ReportData {
        commits: stats_list,
        file_paths,
        github_prs,
    };

    let file = File::create(output_path).context("Failed to create output file")?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &report_data).context("Failed to write JSON")?;

    println!("Collected stats for {} commits into {:?}", report_data.commits.len(), output_path);
    Ok(())
}

fn is_excluded(path: &str, exclude_patterns: &[String]) -> bool {
    for pattern in exclude_patterns {
        // 0. Handle root-relative patterns (starting with /)
        if pattern.starts_with('/') {
            let root_pattern = &pattern[1..];
            if path == root_pattern {
                return true;
            }
            continue;
        }

        // 1. Directory prefix
        if pattern.ends_with('/') {
            if path.starts_with(pattern) {
                return true;
            }
        } 
        // 2. Exact match
        else if path == pattern {
            return true;
        }
        // 3. Filename match anywhere (if no slash in pattern)
        else if !pattern.contains('/') {
            let filename = path.split('/').last().unwrap_or("");
            if filename == pattern {
                return true;
            }
        }
        
        // 4. Basic glob support: handle * at the beginning or end
        if pattern.starts_with('*') && path.ends_with(&pattern[1..]) {
            return true;
        }
        if pattern.ends_with('*') && path.starts_with(&pattern[..pattern.len()-1]) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_excluded() {
        let exclude = vec![
            "target/".to_string(),
            "*.lock".to_string(),
            "secret.txt".to_string(),
            "vendor/*".to_string(),
            "/README.md".to_string(),
        ];

        // Directory prefix
        assert!(is_excluded("target/debug/main", &exclude));
        assert!(is_excluded("target/release/gitpulse", &exclude));
        assert!(!is_excluded("src/target/file", &exclude));

        // Suffix wildcard
        assert!(is_excluded("Cargo.lock", &exclude));
        assert!(is_excluded("package-lock.json.lock", &exclude));
        assert!(!is_excluded("lock.txt", &exclude));

        // Filename match anywhere (no slash in pattern)
        assert!(is_excluded("secret.txt", &exclude));
        assert!(is_excluded("src/secret.txt", &exclude));
        assert!(is_excluded("deep/nesting/secret.txt", &exclude));
        assert!(!is_excluded("my_secret.txt", &exclude));

        // Prefix wildcard
        assert!(is_excluded("vendor/lib.c", &exclude));
        assert!(is_excluded("vendor/sub/file", &exclude));
        assert!(!is_excluded("src/vendor/file", &exclude));

        // Root-relative exact match (starts with /)
        assert!(is_excluded("README.md", &exclude));
        assert!(!is_excluded("docs/README.md", &exclude));
    }
}
