use serde::{Serialize, Deserialize};
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct Inbound {
    pub id: usize,
    pub context: String,
    pub condition: String,
    pub dest_extension: String,
}