use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportData {
    pub commits: Vec<CommitStats>,
    pub file_paths: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardData {
    pub daily_stats: Vec<DailyStat>,
    pub file_stats: Vec<FileStat>,
    pub merge_events: Vec<MergeEvent>,
    pub daily_dir_counts: Vec<DirCount>,
    pub file_paths: Vec<String>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct FileStat {
    pub file_idx: usize,
    pub author: String,
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MergeEvent {
    pub branch: String,
    pub days: u32,
    pub date: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommitStats {
    pub hash: String,
    pub author: String,
    pub date: DateTime<Utc>,
    pub added: usize,
    pub deleted: usize,
    pub email: String,
    pub is_merge: bool,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub files: Vec<usize>, // Indices into file_paths
}
