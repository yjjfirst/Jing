use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FirewallRule {
    pub id: i32,
    pub ip_address: String,
    pub action: String,
    pub created_at: DateTime<Local>,
}
