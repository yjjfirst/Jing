use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Ivr {
    pub id: usize,
    pub exten: String,
    pub name: String,
    pub domain_id: usize,
}
#[derive(Clone, Debug, PartialEq,Serialize, Deserialize)]
pub struct IvrEntry {
    pub id: i32,
    pub ivr_id: i32,
    pub digits: String,
    pub dest_exten: String
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IvrAttr {
    pub id: i32,
    pub ivr_id: i32,
    pub name: String,
    pub value: String
}
#[derive(Clone, Debug, PartialEq,Serialize, Deserialize)]
pub struct IvrAllData {
    pub ivr: Ivr,
    pub attrs: HashMap<String, IvrAttr>,
    pub entries: Vec<IvrEntry>
}

impl IvrAllData {
    pub fn new() -> IvrAllData {
        IvrAllData {
            ivr: Ivr {
                id: 0,
                exten: "".to_string(),
                name: "".to_string(),
                domain_id: 0
            },
            attrs: HashMap::new(),
            entries: vec![]
        }
    }
}