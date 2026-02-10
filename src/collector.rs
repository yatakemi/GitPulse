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

        if !is_merge {
            if commit.parent_count() == 0 {
                // Initial commit
                if let Ok(tree) = commit.tree() {
                    let diff = repo.diff_tree_to_tree(None, Some(&tree), None)?;
                    let stats = diff.stats()?;
                    added = stats.insertions();
                    deleted = stats.deletions();
                }
            } else {
                let parent = commit.parent(0)?;
                let tree = commit.tree()?;
                let parent_tree = parent.tree()?;
                let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), None)?;
                let stats = diff.stats()?;
                added = stats.insertions();
                deleted = stats.deletions();
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
        });
    }

    let file = File::create(output_path).context("Failed to create output file")?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &stats_list).context("Failed to write JSON")?;

    println!("Collected stats for {} commits into {:?}", stats_list.len(), output_path);
    Ok(())
}
