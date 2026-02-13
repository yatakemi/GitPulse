use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use crate::github::GitHubPR;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportData {
    pub commits: Vec<CommitStats>,
    #[serde(default)]
    pub file_paths: Vec<String>,
    #[serde(default)]
    pub github_prs: Vec<GitHubPR>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardData {
    pub daily_stats: Vec<DailyStat>,
    pub file_stats: Vec<FileStat>,
    pub file_type_stats: Vec<FileTypeStat>,
    pub merge_events: Vec<MergeEvent>,
    pub daily_dir_counts: Vec<DirCount>,
    pub weekly_stats: Vec<WeeklyStat>,
    pub forecast: Option<ForecastData>,
    pub github_prs: Vec<GitHubPR>,
    pub file_paths: Vec<String>,
    pub commits: Vec<CommitStats>,
    pub events: Vec<crate::config::EventConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileTypeStat {
    pub extension: String,
    pub added: usize,
    pub deleted: usize,
    pub commits: usize,
    pub churn: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirCount {
    pub date: String,
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DailyStat {
    pub date: String,
    pub author: String,
    pub added: usize,
    pub deleted: usize,
    pub commits: usize,
    pub churn: usize,
    pub merges: usize,
    pub hours: Vec<u32>,
    pub commit_sizes: Vec<usize>, // total_changes of each commit
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WeeklyStat {
    pub week_start: String, // YYYY-MM-DD (Monday)
    pub added: usize,
    pub deleted: usize,
    pub commits: usize,
    pub churn: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForecastData {
    pub current_velocity: f64, // avg commits per week (last 4 weeks)
    pub velocity_trend: f64,   // percentage change
    pub projected_60_days: usize,
    pub est_completion_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileStat {
    pub file_idx: usize,
    pub author: String,
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MergeEvent {
    pub branch: String,
    pub author: String,
    pub days: u32,
    pub date: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommitStats {
    pub hash: String,
    pub author: String,
    pub date: DateTime<FixedOffset>,
    pub added: usize,
    pub deleted: usize,
    pub email: String,
    pub is_merge: bool,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub files: Vec<usize>, // Indices into file_paths
    #[serde(default)]
    pub lead_time_days: Option<u32>,
}
