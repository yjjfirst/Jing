pub mod user;
pub mod ringing_group;
pub mod domain;
pub mod gateway;
pub mod route_out;
pub mod route_in;
pub mod cdr;
pub mod sound_file;
pub mod sound;
pub mod extension;
pub mod conference;
pub mod ivr;
pub mod callcenter;

use gloo_net::http::Request;
use gloo_net::Error;
use serde::{Serialize, de::DeserializeOwned, Deserialize};
use web_sys::FormData;
#[derive(Serialize, Deserialize)]
pub struct EmptyJson{

}
const BASE_URL: &str = "http://telman.me:9090/api";

pub struct Service {}
impl Service {
    pub fn endpoint (url: &str, domain: usize) -> String {
        format!("{}/{}{}",BASE_URL,domain, url)
    }

    pub async fn index<T: Serialize + DeserializeOwned>(path: &str,domain: usize) -> Result<Vec<T>, Error> {
        let endpoint = Self::endpoint(path, domain);
        let response = Request::get(&endpoint)
            .send()
            .await?;
        
        response
            .json()
            .await
    }

    pub async fn get<T: Serialize + DeserializeOwned>(path: &str, domain: usize) -> Result<T, Error> {
        let endpoint = Self::endpoint(path, domain);
        let response = Request::get(&endpoint)
            .send()
            .await?;

        response
            .json()
            .await
    }

    pub async fn post<T: Serialize + DeserializeOwned>(path: &str, domain: usize, group: T) 
    -> Result<EmptyJson, Error>{
        let endpoint = Self::endpoint(path, domain);
        let request = Request::post(&endpoint)
            .json(&group)?;

        let response = request
            .send()
            .await?;

        response
            .json()
            .await
    }

    pub async fn patch<T: Serialize + DeserializeOwned>(path: &str, domain: usize, group: T) 
    -> Result<EmptyJson, Error>{
        let endpoint = Self::endpoint(path, domain);
        let request = Request::patch(&endpoint)
            .json(&group)?;
        let response = request
            .send()
            .await?;
        
        response
            .json()
            .await
    }

    pub async fn post_form(path: &str, domain: usize, form_data: FormData) 
    -> Result<EmptyJson, Error>{
        let endpoint = Self::endpoint(path, domain);
        let request = Request::post(&endpoint)
            .body(form_data)?;

            let response = request
            .send()
            .await?;
        
        response
            .json()
            .await
    }    
    pub async fn delete(path: &str, domain: usize) -> Result<EmptyJson, Error> {
        let endpoint = Self::endpoint(path, domain);
        let response = Request::delete(&endpoint)
            .send()
            .await?;

        response
            .json()
            .await
    }
}
