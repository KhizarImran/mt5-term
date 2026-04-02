use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    pub balance: f64,
    pub equity: f64,
    pub margin: f64,
    pub free_margin: f64,
    pub profit: f64,
}

impl AccountInfo {
    #[allow(dead_code)]
    pub fn margin_level(&self) -> f64 {
        if self.margin > 0.0 {
            (self.equity / self.margin) * 100.0
        } else {
            0.0
        }
    }
}
