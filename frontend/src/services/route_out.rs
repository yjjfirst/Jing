use serde::{Serialize, Deserialize};
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct Outbound {
    pub id: usize,
    pub priority: usize,
    pub condition: String,
    pub gateway_id: usize,
}