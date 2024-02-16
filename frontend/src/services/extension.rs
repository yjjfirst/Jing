use gloo_net::http::Request;
use yew::Properties;
use serde::{Serialize, Deserialize};

use super::BASE_URL;

#[derive(Clone, PartialEq, Deserialize, Properties, Serialize, Debug)]
pub struct Extension {
    pub id: usize,
    pub domain_id: i32,
    pub user_id: String
}

impl Extension {
    pub async fn list(domain: usize) -> Vec<String> {
        let endpoint = format!("{}/{}/extension", BASE_URL, domain);
        let response = Request::get(&endpoint).send().await.unwrap();

        let extensions: Vec<Extension> = response.json().await.unwrap();

        extensions.iter().map(|e|{
            e.user_id.to_string()
        }).collect::<Vec<String>>()
    }
}