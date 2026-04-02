use anyhow::Result;
use chrono::Utc;
use std::path::Path;

use crate::models::Mt5Data;

pub struct App {
    pub data: Option<Mt5Data>,
    pub should_quit: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
}

impl App {
    pub fn new() -> Self {
        Self {
            data: None,
            should_quit: false,
        }
    }

    pub fn load_data(&mut self, path: &Path) -> Result<()> {
        let content = std::fs::read_to_string(path)?;
        let data: Mt5Data = serde_json::from_str(&content)?;
        self.data = Some(data);
        Ok(())
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn connection_status(&self) -> ConnectionStatus {
        match &self.data {
            Some(data) => {
                let now = Utc::now();
                let age = now.signed_duration_since(data.timestamp);

                // If data is older than 10 seconds, consider disconnected
                if age.num_seconds() > 10 {
                    ConnectionStatus::Disconnected
                } else {
                    ConnectionStatus::Connected
                }
            }
            None => ConnectionStatus::Disconnected,
        }
    }

    pub fn last_update(&self) -> Option<String> {
        self.data.as_ref().map(|data| {
            let now = Utc::now();
            let age = now.signed_duration_since(data.timestamp);

            if age.num_seconds() < 60 {
                format!("{}s ago", age.num_seconds())
            } else if age.num_minutes() < 60 {
                format!("{}m ago", age.num_minutes())
            } else {
                format!("{}h ago", age.num_hours())
            }
        })
    }
}
