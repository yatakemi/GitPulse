use crate::model::CommitStats;
use crate::html_template::HTML_TEMPLATE;
use anyhow::{Context, Result};
use chrono::{Datelike, NaiveDate};
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use tera::{Context as TeraContext, Tera};

#[derive(Debug, Serialize, Clone)]
pub struct AggregatedStats {
    pub date: String,
    pub user: String,
    pub added: usize,
    pub deleted: usize,
    pub total_changes: usize,
    pub commit_count: usize,
}

pub fn visualize_stats(input_path: &Path, output_path: &Path, format: &str) -> Result<()> {
    let file = File::open(input_path).context("Failed to open input file")?;
    let reader = BufReader::new(file);
    let commits: Vec<CommitStats> = serde_json::from_reader(reader).context("Failed to parse JSON")?;

    // Aggregate data by day and user: (added, deleted, commit_count)
    let mut aggregation: HashMap<(String, String), (usize, usize, usize)> = HashMap::new();

    for commit in &commits {
        // Group by day
        let date = commit.date.date_naive();
        let day_str = date.format("%Y-%m-%d").to_string();

        let key = (day_str, commit.author.clone());
        let entry = aggregation.entry(key).or_insert((0, 0, 0));
        entry.0 += commit.added;
        entry.1 += commit.deleted;
        entry.2 += 1; // Increment commit count
    }

    let mut stats_list: Vec<AggregatedStats> = aggregation
        .into_iter()
        .map(|((date, user), (added, deleted, commit_count))| AggregatedStats {
            date,
            user,
            added,
            deleted,
            total_changes: added + deleted,
            commit_count,
        })
        .collect();

    // Sort by date
    stats_list.sort_by(|a, b| a.date.cmp(&b.date));

    match format {
        "csv" => export_csv(&stats_list, output_path),
        "html" => export_html(&commits, output_path),
        _ => anyhow::bail!("Unsupported format: {}", format),
    }
}

fn export_csv(stats: &[AggregatedStats], output_path: &Path) -> Result<()> {
    let mut wtr = csv::Writer::from_path(output_path)?;
    for stat in stats {
        wtr.serialize(stat)?;
    }
    wtr.flush()?;
    println!("Exported CSV to {:?}", output_path);
    Ok(())
}

fn export_html(stats: &[CommitStats], output_path: &Path) -> Result<()> {
    let mut context = TeraContext::new();
    context.insert("data", stats);
    let rendered = Tera::one_off(crate::html_template::HTML_TEMPLATE, &context, false)
        .context("Failed to render HTML template")?;
    let mut file = File::create(output_path)?;
    file.write_all(rendered.as_bytes())?;
    println!("Exported HTML to {:?}", output_path);
    Ok(())
}
