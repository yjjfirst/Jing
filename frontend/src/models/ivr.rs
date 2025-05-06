use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use web_sys::FormData;
use util_macro::HashMapHelper;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Ivr {
    pub id: usize,
    pub exten: String,
    pub name: String,
    pub domain_id: usize,
    pub attrs: HashMap<String, IvrAttr>,
    pub entries: Vec<IvrEntry>
}

impl Ivr {
    pub fn new() -> Ivr {
        Ivr {
            id: 0,
            exten: "".to_string(),
            name: "".to_string(),
            domain_id: 0,
            attrs: HashMap::new(),
            entries: vec![]
        }
    }
}

#[derive(Clone, Debug, PartialEq,Serialize, Deserialize)]
pub struct IvrEntry {
    pub id: i32,
    pub ivr_id: usize,
    pub digits: String,
    pub dest_exten: String
}

impl IvrEntry {
    pub fn new() -> IvrEntry {
        IvrEntry {
            id: 0,
            ivr_id: 0,
            digits: "".to_string(),
            dest_exten: "".to_string()
        }
    } 
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, HashMapHelper)]
pub struct IvrAttr {
    pub id: i32,
    pub ivr_id: usize,
    pub name: String,
    pub value: String
}

impl IvrAttr {
    pub fn new() -> IvrAttr {
        IvrAttr {
            id: 0,
            ivr_id: 0,
            name: "".to_string(),
            value: "".to_string()
        }
    }    
}