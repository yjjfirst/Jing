use serde::{Serialize, Deserialize};
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct Outbound {
    pub id: usize,
    pub priority: usize,
    pub condition: String,
    pub gateway_id: usize,
    pub prepend: String,
    pub prefix: usize
}

impl Outbound {
    pub fn new() -> Self {
        Outbound {
            id: 0,
            priority: 100,
            condition: "".to_string(),
            gateway_id: 0,
            prepend: "".to_string(),
            prefix: 0
        }
    }
}