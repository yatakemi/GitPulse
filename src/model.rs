use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitStats {
    pub hash: String,
    pub author: String,
    pub date: DateTime<Utc>,
    pub added: usize,
    pub deleted: usize,
    pub email: String,
    pub is_merge: bool,
}
