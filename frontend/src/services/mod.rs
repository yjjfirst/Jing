pub mod user;
pub mod ringing_group;
pub mod domain;
pub mod gateway;
pub mod route_out;
pub mod route_in;
pub mod cdr;
pub mod sound_file;

use gloo_net::http::Request;
use gloo_net::Error;
use serde::{Serialize, de::DeserializeOwned, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct EmptyJson{

}
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

    pub async fn update<T: Serialize + DeserializeOwned>(path: &str, domain: usize, group: T) 
    -> Result<EmptyJson, Error>{
        let endpoint = Self::endpoint(path, domain);
        let request = Request::post(&endpoint).json(&group).unwrap();
        let response = request.send().await.unwrap();
        return response.json().await
    }

    pub async fn delete(path: &str, domain: usize) {
        let endpoint = Self::endpoint(path, domain);
        Request::delete(&endpoint).send().await.unwrap();
    }
}
