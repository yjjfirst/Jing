use yew::Properties;
use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Deserialize, Properties, Serialize, Debug)]
pub struct RingGroup {
    pub id: usize,
    pub name: String,
    pub group_id: String,
    pub description: String,
    pub domain_id: usize,
    pub ring_time: i32,
    pub ring_strategy: String,
    pub members: Vec<String>
}

impl RingGroup {
    pub fn new_empty() -> RingGroup {
        RingGroup  {
            id: 0,
            name: "".to_string(),
            group_id: "".to_string(),
            description: "".to_string(),
            ring_time: 0,
            domain_id: 1,
            ring_strategy: "all".to_string(),
            members: vec![]
        }
    }
    pub fn new(id: usize,
        name: String,
        group_id: String,
        description: String,
        domain_id: usize,
        ring_time: i32,
        ring_strategy: String,
        members: Vec<String>) -> RingGroup {
        RingGroup {
            id,
            name,
            group_id,
            description,
            domain_id,
            ring_time,
            ring_strategy,
            members
        }
    }
}
