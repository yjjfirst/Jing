use yew::Properties;
use gloo_net::http::Request;
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Properties)]
pub struct RingingGroup {
    pub id: usize,
    pub name: String,
    pub group_id: String,
    pub description: Option<String>,
    pub ring_time: i32,
    pub ring_strategy: String
}

impl RingingGroup {
    pub fn new() -> RingingGroup {
        RingingGroup {
            id: 0,
            name: "".to_string(),
            group_id: "".to_string(),
            description: None,
            ring_time: 0,
            ring_strategy: "all".to_string()       
        }
    }
    pub async fn fetch_all() -> Vec<RingingGroup> {
        let endpoint = format!("http://teleman.me:9090/api/ringing-groups");
        Request::get(&endpoint)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }
    pub async fn fetch(id: i32) -> RingingGroup {
        let endpoint = format!("http://teleman.me:9090/api/ringing-groups/{}", id);
        Request::get(&endpoint)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }
    
}



