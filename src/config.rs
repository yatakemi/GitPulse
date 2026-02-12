use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub alias: HashMap<String, String>,
    #[serde(default)]
    pub exclude: Vec<String>,
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
