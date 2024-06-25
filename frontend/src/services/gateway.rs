use gloo_net::http::Request;
use serde::{Serialize, Deserialize};
use super::BASE_URL;

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct Gateway {
    pub id: usize,
    pub gateway_name: String,
    pub proxy: String,
    pub register: String,
    pub username: String,
    pub password: String,
    pub profile_id: usize,
}

impl Gateway {
    pub fn new() -> Gateway {
        Gateway {
            id: 0, 
            profile_id: 0,
            proxy: "".to_string(), 
            register: "".to_string(),
            gateway_name: "".to_string(),
            username: "".to_string(),
            password: "".to_string()        
       }
    }

    pub async fn get(domain: usize, id: usize) -> Gateway {
        let endpoint = format!("{}/{}/gateway/{}", BASE_URL, domain, id);
        let response = Request::get(&endpoint).send().await.unwrap();

        let gateway: Gateway = response.json().await.unwrap();

        gateway
    }
}