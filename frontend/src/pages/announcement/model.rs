use serde::{Serialize, Deserialize};
use crate::pages::recording::model::Recording;

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct Announcement {
    pub id: usize,
    pub name: String,
    pub exten: String,
    pub domain_id: usize,
    pub recording: Recording
}

impl Announcement {
    pub fn new() -> Announcement {
        Announcement {
            id: 0,
            name: "".to_string(),
            exten: "".to_string(),
            domain_id: 0,
            recording: Recording::new(),
        }
    }
}