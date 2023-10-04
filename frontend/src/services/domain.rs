use yew::Properties;
use gloo_net::http::{Request, Response};
use serde::{Deserialize, Serialize};

use super::BASE_URL;

#[derive(Clone, PartialEq, Deserialize, Properties, Serialize)]
pub struct Domain {
    pub id: usize,
    pub domain_name: String
}

impl Domain {
    pub async fn index() ->Vec<Domain> {
        let endpoint = format!("{}/domain", BASE_URL);
        let response = Request::get(&endpoint).send().await.unwrap();
        
        response.json().await.unwrap()
    }
}