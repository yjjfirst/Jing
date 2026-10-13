use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct Profile {
    pub id: usize,
    pub name: String,
    pub param_helps: Vec<ProfileParamHelp>,
    pub params: HashMap<String, Param>
}

impl Profile {
    pub fn new() -> Profile {
        Profile { id: 0,
            name: "".to_string(),
            param_helps: Vec::new(),
            params: HashMap::new()
        }
    }
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct ProfileParamHelp {
    pub id: i32,
    pub name: String,
    pub range_text: String,
    pub help_text: String,
    pub readonly: bool,
}


#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct Param {
    pub id: usize,
    pub profile_id: usize,
    pub name: String,
    pub value: String,
    pub ignored: bool,
}

impl Param {
    pub fn new() -> Param {
        Param {
            id: 0,
            profile_id: 0,
            name: "".to_string(),
            value: "".to_string(),
            ignored: false
        }
    }
    pub fn get(name: &str, params: &HashMap<String, Param>) -> String {
        match params.get(name) {
            Some(p) => p.value.clone(),
            None => "".to_string(),
        }
    }

    pub fn ignored(name: &str, params: &HashMap<String, Param>) -> bool {
        match params.get(name) {
            Some(p) => p.ignored,
            None => false,
        }
    }
}
