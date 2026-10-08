use serde::{Serialize, Deserialize};
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct SoundFile {
    pub id: usize,
    pub name: String,
    pub domain_id: usize,
    pub description: String
}

impl SoundFile {
    pub fn new() -> SoundFile {
        SoundFile {
            id: 0,
            name: "".to_string(),
            domain_id: 0,
            description: "".to_string()
        }
    }
}