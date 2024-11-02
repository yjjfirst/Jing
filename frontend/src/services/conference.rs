use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Conf {
    pub id: usize,
    pub exten: String,
    pub name: String,
    pub description: String,
    pub domain_id: usize,
    pub conference_profile_id: usize
}

impl Conf {
    pub fn new() -> Conf {
        Conf {
            id: 0,
            exten: "".to_string(),
            name: "".to_string(),
            description: "".to_string(),
            domain_id: 0,
            conference_profile_id: 0
        }
    }
}