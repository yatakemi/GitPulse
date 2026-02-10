use crate::model::CommitStats;
use anyhow::{Context, Result};
use chrono::{DateTime, TimeZone, Utc};
use git2::{Repository, Sort};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub fn collect_stats(repo_path: &Path, output_path: &Path) -> Result<()> {
    let repo = Repository::open(repo_path).context("Failed to open repository")?;
    let mut revwalk = repo.revwalk()?;
    revwalk.set_sorting(Sort::TIME)?;
    revwalk.push_head()?;

    let mut stats_list = Vec::new();
    let mut file_paths = Vec::new();
    let mut file_map: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    for oid_result in revwalk {
        let oid = oid_result?;
        let commit = repo.find_commit(oid)?;

        // Check if it's a merge commit
        let is_merge = commit.parent_count() > 1;

        let author = commit.author();
        let author_name = author.name().unwrap_or("Unknown").to_string();
        let author_email = author.email().unwrap_or("").to_string();
        
        let time = commit.time();
        let date: DateTime<Utc> = Utc.timestamp_opt(time.seconds(), 0).unwrap();

        let mut added = 0;
        let mut deleted = 0;
        let mut commit_files = Vec::new();

        if !is_merge {
            if commit.parent_count() == 0 {
                // Initial commit
                if let Ok(tree) = commit.tree() {
                    let diff = repo.diff_tree_to_tree(None, Some(&tree), None)?;
                    let stats = diff.stats()?;
                    added = stats.insertions();
                    deleted = stats.deletions();
                    
                    // Collect file paths
                    diff.foreach(&mut |delta, _float| {
                        if let Some(path) = delta.new_file().path().and_then(|p| p.to_str()) {
                            let path_string = path.to_string();
                            let idx = if let Some(&i) = file_map.get(&path_string) {
                                i
                            } else {
                                let i = file_paths.len();
                                file_paths.push(path_string.clone());
                                file_map.insert(path_string, i);
                                i
                            };
                            commit_files.push(idx);
                        }
                        true
                    }, None, None, None)?;
                }
            } else {
                let parent = commit.parent(0)?;
                let tree = commit.tree()?;
                let parent_tree = parent.tree()?;
                let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), None)?;
                let stats = diff.stats()?;
                added = stats.insertions();
                deleted = stats.deletions();

                // Collect file paths
                diff.foreach(&mut |delta, _float| {
                    if let Some(path) = delta.new_file().path().and_then(|p| p.to_str()) {
                        let path_string = path.to_string();
                        let idx = if let Some(&i) = file_map.get(&path_string) {
                            i
                        } else {
                            let i = file_paths.len();
                            file_paths.push(path_string.clone());
                            file_map.insert(path_string, i);
                            i
                        };
                        commit_files.push(idx);
                    }
                    true
                }, None, None, None)?;
            }
        }

        stats_list.push(CommitStats {
            hash: oid.to_string(),
            author: author_name,
            date,
            added,
            deleted,
            email: author_email,
            is_merge,
            files: commit_files,
        });
    }

    let report_data = crate::model::ReportData {
        commits: stats_list,
        file_paths,
    };

    let file = File::create(output_path).context("Failed to create output file")?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &report_data).context("Failed to write JSON")?;

    println!("Collected stats for {} commits into {:?}", report_data.commits.len(), output_path);
    Ok(())
}
