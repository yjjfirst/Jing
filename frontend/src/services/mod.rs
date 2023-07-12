use yew::Properties;
use gloo_net::http::{Request, Response};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Properties, Serialize)]
pub struct RingingGroup {
    pub id: usize,
    pub name: String,
    pub group_id: String,
    pub description: Option<String>,
    pub domain_id: i32,
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
            domain_id: 1,
            ring_strategy: "all".to_string()       
        }
    }
    pub fn new(id: usize, 
        name: String, 
        group_id: String, 
        description: Option<String>, 
        domain_id: i32,
        ring_time: i32, 
        ring_strategy: String) -> RingingGroup {
            RingingGroup {id, name, group_id, description, domain_id, ring_time, ring_strategy}
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
        let request = Request::post(&endpoint).json(&group).unwrap();
        let response: Response = request.send().await.unwrap();
        response.json().await.unwrap()
    }
    
}



