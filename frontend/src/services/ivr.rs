use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Ivr {
    pub id: usize,
    pub exten: String,
    pub name: String,
    pub domain_id: usize,
}

impl Ivr {
    pub fn new() -> Ivr {
        Ivr {
            id: 0,
            exten: "".to_string(),
            name: "".to_string(),
            domain_id: 0
        }
    }
}