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
}

pub fn visualize_stats(input_path: &Path, output_path: &Path, format: &str) -> Result<()> {
    let file = File::open(input_path).context("Failed to open input file")?;
    let reader = BufReader::new(file);
    let commits: Vec<CommitStats> = serde_json::from_reader(reader).context("Failed to parse JSON")?;

    // Aggregate data by week and user
    let mut aggregation: HashMap<(String, String), (usize, usize)> = HashMap::new();

    for commit in commits {
        // Group by week (start of the week)
        let date = commit.date.date_naive();
        let week_str = date.format("%Y-%W").to_string(); // e.g., 2025-06
        // Or simpler: just use the date. But for productivity, weekly is often better. 
        // Let's use daily for now as per plan "Daily/Weekly", but implementation plan said "Group by User and Time".
        // Let's stick to Daily for higher resolution in the graph, users can zoom out.
        let day_str = date.format("%Y-%m-%d").to_string();

        let key = (day_str, commit.author);
        let entry = aggregation.entry(key).or_insert((0, 0));
        entry.0 += commit.added;
        entry.1 += commit.deleted;
    }

    let mut stats_list: Vec<AggregatedStats> = aggregation
        .into_iter()
        .map(|((date, user), (added, deleted))| AggregatedStats {
            date,
            user,
            added,
            deleted,
            total_changes: added + deleted,
        })
        .collect();

    // Sort by date
    stats_list.sort_by(|a, b| a.date.cmp(&b.date));

    match format {
        "csv" => export_csv(&stats_list, output_path),
        "html" => export_html(&stats_list, output_path),
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

fn export_html(stats: &[AggregatedStats], output_path: &Path) -> Result<()> {
    let mut tera = Tera::default();
    tera.add_raw_template("report.html", HTML_TEMPLATE)?;

    let mut context = TeraContext::new();
    context.insert("data", stats);

    let rendered = tera.render("report.html", &context)?;
    let mut file = File::create(output_path)?;
    file.write_all(rendered.as_bytes())?;
    
    println!("Exported HTML to {:?}", output_path);
    Ok(())
}
