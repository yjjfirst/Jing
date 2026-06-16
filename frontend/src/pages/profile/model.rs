use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct Profile {
    pub id: usize,
    pub name: String,
    pub params: HashMap<String, Param>
}

impl Profile {
    pub fn new() -> Profile {
        Profile { id: 0, name: "".to_string(), params: HashMap::new() }
    }
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct Param {
    pub id: usize,
    pub profile_id: usize,
    pub name: String,
    pub value: String,
}

impl Param {
    pub fn get(name: &str, params: &HashMap<String, Param>) -> String {
        match params.get(name) {
            Some(p) => p.value.clone(),
            None => "".to_string(),
        }
    }
}
