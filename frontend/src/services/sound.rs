use serde::{Serialize, Deserialize};
use super::sound_file::SoundFile;

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct Sound {
    pub id: usize,
    pub name: String,
    pub exten: String,
    pub domain_id: usize,
    pub sound_file_id: usize
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct ApiSound {
    pub sound: Sound,
    pub sound_file: SoundFile
}

impl Sound {
    pub fn new() -> Sound {
        Sound {
            id: 0,
            name: "".to_string(),
            exten: "".to_string(),
            domain_id: 0,
            sound_file_id: 0,
        }
    }
}