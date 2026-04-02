use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub mt5: Mt5Config,
    pub data: DataConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mt5Config {
    pub terminal_path: Option<String>,
    pub account: Option<u64>,
    pub password: Option<String>,
    pub server: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataConfig {
    pub output_dir: String,
    pub update_interval_seconds: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mt5: Mt5Config {
                terminal_path: None,
                account: None,
                password: None,
                server: None,
            },
            data: DataConfig {
                output_dir: "./data".to_string(),
                update_interval_seconds: 5,
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path();
        let content = std::fs::read_to_string(&config_path)
            .context("Failed to read config.json")?;
        let config: Config = serde_json::from_str(&content)
            .context("Failed to parse config.json")?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path();
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;
        Ok(())
    }

    pub fn exists() -> bool {
        Self::config_path().exists()
    }

    pub fn config_path() -> PathBuf {
        PathBuf::from("config.json")
    }

    pub fn find_mt5_terminal() -> Option<String> {
        let paths = vec![
            r"C:\Program Files\MetaTrader 5\terminal64.exe",
            r"C:\Program Files (x86)\MetaTrader 5\terminal64.exe",
        ];

        for path in paths {
            if Path::new(path).exists() {
                return Some(path.to_string());
            }
        }

        None
    }
}
