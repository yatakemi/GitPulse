mod collector;
mod config;
mod html_template;
mod model;
mod visualizer;

use anyhow::Context;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Collect stats from a git repository and save to JSON
    Collect {
        /// Path to the git repository
        #[arg(short, long, default_value = ".")]
        repo: PathBuf,

        /// Output JSON path
        #[arg(short, long, default_value = "git_stats.json")]
        out: PathBuf,

        /// Analyze only merge commits
        #[arg(short, long, default_value_t = false)]
        merges_only: bool,
    },
    /// Visualize stats from JSON to CSV or HTML
    Visualize {
        /// Input JSON path
        #[arg(short, long, default_value = "git_stats.json")]
        data: PathBuf,

        /// Output file path
        #[arg(short, long, default_value = "report.html")]
        out: PathBuf,

        /// Output format (csv or html)
        #[arg(short, long, default_value = "html")]
        format: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Collect { repo, out, merges_only } => {
            let config_path = std::path::Path::new("gitpulse.toml");
            let config = if config_path.exists() {
                config::Config::load(config_path)
                    .context(format!("Failed to parse config file: {:?}", config_path))?
            } else {
                config::Config::default()
            };
            collector::collect_stats(repo, out, &config, *merges_only)?;
        }
        Commands::Visualize { data, out, format } => {
            visualizer::visualize_stats(data, out, format)?;
        }
    }

    Ok(())
}
