use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use util_macro::HashMapHelper;
use web_sys::FormData;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Agent {
    pub id: usize,
    pub user_id: usize,
    pub domain_id: usize,
    pub name: String,
    pub contact: String,
    pub leg_timeout: i32,
    pub params: HashMap<String, AgentParam>
}

impl Agent {
    pub fn new() -> Agent {
        Agent {
            id: 0,
            user_id: 0,
            domain_id: 0,
            name: "".to_string(),
            contact: "".to_string(),
            leg_timeout: 30,
            params: HashMap::new()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, HashMapHelper)]
pub struct AgentParam {
    pub id: i32,
    pub agent_id: i32,
    pub name: String,
    pub value: String,
}

impl AgentParam {
    pub fn _new() -> AgentParam {
        AgentParam { 
            id: 0, 
            agent_id: 0, 
            name: "".to_string(), 
            value: "".to_string(),
        }
    }
}