use anyhow::Result;
use std::path::Path;

use crate::models::Mt5Data;

pub struct App {
    pub data: Option<Mt5Data>,
    pub should_quit: bool,
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
}
