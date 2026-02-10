use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportData {
    pub commits: Vec<CommitStats>,
    pub file_paths: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
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
