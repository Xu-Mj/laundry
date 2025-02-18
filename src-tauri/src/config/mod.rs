use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::error::Result;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub log: LogConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LogConfig {
    pub level: String,
    pub output: String,
}

impl LogConfig {
    pub fn level(&self) -> tracing::Level {
        match self.level.as_str() {
            "trace" => tracing::Level::TRACE,
            "debug" => tracing::Level::DEBUG,
            "info" => tracing::Level::INFO,
            "warn" => tracing::Level::WARN,
            "error" => tracing::Level::ERROR,
            _ => tracing::Level::INFO,
        }
    }
}

impl Config {
    pub fn load(filename: impl AsRef<Path>) -> Result<Self> {
        let content = fs::read_to_string(filename)?;
        Ok(serde_yaml::from_str(&content)?)
    }
}
