use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{AccountInfo, Position};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mt5Data {
    pub timestamp: DateTime<Utc>,
    pub account: AccountInfo,
    pub positions: Vec<Position>,
}
