use serde::{Serialize, Deserialize};
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct Recording {
    pub id: usize,
    pub name: String,
    pub domain_id: usize,
    pub description: String
}

impl Recording {
    pub fn new() -> Recording {
        Recording {
            id: 0,
            name: "".to_string(),
            domain_id: 0,
            description: "".to_string()
        }
    }
}