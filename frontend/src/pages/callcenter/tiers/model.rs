use serde::{Deserialize, Serialize};
use super::super::agents::model::Agent;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Tier {
    pub id: i32,
    pub queue_id: i32,
    pub level: i32,
    pub position: i32,
    pub agent: Agent
}
