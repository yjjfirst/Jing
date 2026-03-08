use yew::Properties;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use gloo_console::log;

use super::API_BASE;

#[derive(Clone, Debug, PartialEq, Deserialize, Properties, Serialize)]
pub struct Domain {
    pub id: usize,
    pub domain_name: String
}

impl Domain {
    pub async fn index() ->Vec<Domain> {
        let endpoint = format!("{}/domain", API_BASE);

        log!(&endpoint);
        let response = Request::get(&endpoint).send().await.unwrap();
        
        response.json().await.unwrap()
    }
}