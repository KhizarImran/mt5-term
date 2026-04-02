use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub ticket: u64,
    pub symbol: String,
    #[serde(rename = "type")]
    pub position_type: String,
    pub volume: f64,
    pub open_price: f64,
    pub current_price: f64,
    pub profit: f64,
    pub open_time: DateTime<Utc>,
}

impl Position {
    pub fn is_profitable(&self) -> bool {
        self.profit > 0.0
    }
}
