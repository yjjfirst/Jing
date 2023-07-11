use yew::Properties;
use gloo_net::http::{Request, Response};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Properties, Serialize)]
pub struct RingingGroup {
    pub id: usize,
    pub name: String,
    pub group_id: String,
    pub description: Option<String>,
    pub ring_time: i32,
    pub ring_strategy: String
}

impl RingingGroup {
    pub fn new_empty() -> RingingGroup {
        RingingGroup {
            id: 0,
            name: "".to_string(),
            group_id: "".to_string(),
            description: None,
            ring_time: 0,
            ring_strategy: "all".to_string()       
        }
    }
    pub fn new(id: usize, 
        name: String, 
        group_id: String, 
        description: Option<String>, 
        ring_time: i32, 
        ring_strategy: String) -> RingingGroup {
            RingingGroup {id, name, group_id, description, ring_time, ring_strategy}
    }

    pub async fn fetch_all() -> Vec<RingingGroup> {
        let endpoint = format!("http://teleman.me:9090/api/ringing-groups");
        let response = Request::get(&endpoint).send().await.unwrap();
        response.json().await.unwrap()
    }
    pub async fn fetch(id: usize) -> RingingGroup {
        let endpoint = format!("http://teleman.me:9090/api/ringing-groups/{}", id);
        let response = Request::get(&endpoint).send().await.unwrap();
        response.json().await.unwrap()
    }

    pub async fn update(id: usize, group: RingingGroup) {
        let endpoint = format!("http://teleman.me:9090/api/ringing-groups/{}", id);
        let data = serde_json::to_string(&group).unwrap();
        let response: Response = Request::post(&endpoint).body(data).send().await.unwrap();
        response.json().await.unwrap()
    }
    
}



