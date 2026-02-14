use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub alias: HashMap<String, String>,
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(default)]
    pub events: Vec<EventConfig>,
    #[serde(default = "default_base_branches")]
    pub base_branches: Vec<String>,
    #[serde(default)]
    pub filter: CommitFilterConfig,
    #[serde(default)]
    pub groups: HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommitFilterConfig {
    pub max_lines: Option<usize>,
    pub max_files: Option<usize>,
    #[serde(default)]
    pub ignore_messages: Vec<String>,
    #[serde(default = "default_ignore_bots")]
    pub ignore_bots: bool,
}

fn default_ignore_bots() -> bool {
    true
}

impl Default for CommitFilterConfig {
    fn default() -> Self {
        Self {
            max_lines: Some(5000), // Default high threshold
            max_files: Some(100),  // Default high threshold
            ignore_messages: vec![],
            ignore_bots: true,
        }
    }
}

fn default_base_branches() -> Vec<String> {
    vec!["main".to_string(), "master".to_string(), "develop".to_string()]
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventConfig {
    pub date: String,
    pub name: String,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Config::default());
        }
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
