use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap};
use util_macro::HashMapHelper;
use web_sys::FormData;

use crate::models::API_BASE;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]

pub struct Queue {
    pub id: i32,
    pub domain_id: i32,
    pub exten: String,
    pub name: String,
    pub params: HashMap<String, QueueParam>
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            id: 0,
            domain_id: 0,
            exten: "".to_string(),
            name: "".to_string(),
            params: HashMap::new(),
        }
    }

    pub async fn list(domain: usize) -> Vec<Queue> {
        let endpoint = format!("{}/{domain}/callcenter/queue", API_BASE);
        let response = Request::get(&endpoint).send().await.unwrap();

        let queues: Vec<Queue> = response.json().await.unwrap();
        queues
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, HashMapHelper)]
pub struct QueueParam {
    pub id: i32,
    pub queue_id: i32,
    pub name: String,
    pub value: String,
}

impl QueueParam {
    pub fn _new() -> QueueParam {
        QueueParam { 
            id: 0, 
            queue_id: 0, 
            name: "".to_string(), 
            value: "".to_string(),
        }
    }
}
