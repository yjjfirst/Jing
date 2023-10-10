pub mod ringing_group;
pub mod domain;

use gloo_net::http::Request;
use serde::{Serialize, de::DeserializeOwned};

const BASE_URL: &str = "http://teleman.me:9090/api";

pub struct Service {}
impl Service {
    fn endpoint (url: &str, domain: usize) -> String {
        format!("{}/{}{}",BASE_URL,domain, url)
    }

    pub async fn index<T: Serialize + DeserializeOwned>(path: &str,domain: usize) -> Vec<T> {
        let endpoint = Self::endpoint(path, domain);
        let response = Request::get(&endpoint).send().await.unwrap();
        response.json().await.unwrap()
    }
    pub async fn get<T: Serialize + DeserializeOwned>(path: &str, domain: usize) -> T {
        let endpoint = Self::endpoint(path, domain);
        let response = Request::get(&endpoint).send().await.unwrap();
        response.json().await.unwrap()
    }

    pub async fn update<T: Serialize + DeserializeOwned>(path: &str, domain: usize, group: T) {
        let endpoint = Self::endpoint(path, domain);
        let request = Request::post(&endpoint).json(&group).unwrap();
        request.send().await.unwrap();
    }
}
