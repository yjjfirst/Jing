use serde::{Serialize, Deserialize};
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct SoundFile {
    pub id: usize,
    pub name: String,
    pub domain_id: usize,
    pub description: String
}