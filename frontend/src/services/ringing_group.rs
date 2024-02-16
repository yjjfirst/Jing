use yew::Properties;
use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Deserialize, Properties, Serialize, Debug)]
pub struct RingingGroupDetail {
    pub id: usize,
    pub name: String,
    pub group_id: String,
    pub description: String,
    pub domain_id: i32,
    pub ring_time: i32,
    pub ring_strategy: String
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct RingingGroup (pub RingingGroupDetail, pub Vec<String>);


impl RingingGroup {
    pub fn new_empty() -> RingingGroup {
        RingingGroup (
            RingingGroupDetail {
                id: 0,
                name: "".to_string(),
                group_id: "".to_string(),
                description: "".to_string(),
                ring_time: 0,
                domain_id: 1,
                ring_strategy: "all".to_string()
            }, vec![])
    }
    pub fn new(id: usize, 
        name: String, 
        group_id: String, 
        description: String, 
        domain_id: i32,
        ring_time: i32, 
        ring_strategy: String,
        members: Vec<String>) -> RingingGroup {
        RingingGroup(
            RingingGroupDetail {
                id, 
                name, 
                group_id, 
                description, 
                domain_id, 
                ring_time, 
                ring_strategy
            }, members)
    }
    
}
