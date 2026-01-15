use crate::entry::DateEntry;
use chrono::NaiveDate;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct RawEntry {
    date: String,
}

#[derive(Debug)]
pub struct Config {
    pub entries: HashMap<String, DateEntry>,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let path = Self::config_path()?;
        let content = fs::read_to_string(&path).map_err(|_| ConfigError::NotFound)?;
        Self::parse(&content)
    }

    pub fn config_path() -> Result<PathBuf, ConfigError> {
        let config_dir = dirs::config_dir().ok_or(ConfigError::NoConfigDir)?;
        Ok(config_dir.join("prl").join("config.toml"))
    }

    fn parse(content: &str) -> Result<Self, ConfigError> {
        let raw: HashMap<String, RawEntry> =
            toml::from_str(content).map_err(|e| ConfigError::ParseError(e.to_string()))?;

        let mut entries = HashMap::new();
        for (name, raw_entry) in raw {
            let date = NaiveDate::parse_from_str(&raw_entry.date, "%Y-%m-%d")
                .map_err(|_| ConfigError::InvalidDate(name.clone(), raw_entry.date.clone()))?;
            entries.insert(name, DateEntry::new(date));
        }

        Ok(Config { entries })
    }

    pub fn get(&self, name: &str) -> Option<&DateEntry> {
        self.entries.get(name)
    }

    pub fn sample() -> &'static str {
        r#"# prl configuration file
# Place this file at ~/.config/prl/config.toml

[birthday]
date = "2025-12-25"

[project_deadline]
date = "2025-06-01"
"#
    }
}

#[derive(Debug)]
pub enum ConfigError {
    NotFound,
    NoConfigDir,
    ParseError(String),
    InvalidDate(String, String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::NotFound => write!(f, "Config file not found"),
            ConfigError::NoConfigDir => write!(f, "Could not determine config directory"),
            ConfigError::ParseError(e) => write!(f, "Failed to parse config: {}", e),
            ConfigError::InvalidDate(name, date) => {
                write!(f, "Invalid date for '{}': {}", name, date)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_config() {
        let toml = r#"
[birthday]
date = "2025-03-15"

[deadline]
date = "2025-06-01"
"#;
        let config = Config::parse(toml).unwrap();
        assert_eq!(config.entries.len(), 2);
        assert!(config.get("birthday").is_some());
        assert!(config.get("deadline").is_some());
    }

    #[test]
    fn parse_invalid_date() {
        let toml = r#"
[event]
date = "invalid"
"#;
        let result = Config::parse(toml);
        assert!(matches!(result, Err(ConfigError::InvalidDate(_, _))));
    }

    #[test]
    fn get_nonexistent_entry() {
        let toml = r#"
[birthday]
date = "2025-03-15"
"#;
        let config = Config::parse(toml).unwrap();
        assert!(config.get("nonexistent").is_none());
    }
}
