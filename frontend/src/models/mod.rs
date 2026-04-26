pub mod domain;
pub mod extension;

use gloo_net::http::Request;
use gloo_net::Error;
use serde::{Serialize, de::DeserializeOwned, Deserialize};
use web_sys::FormData;
#[derive(Serialize, Deserialize)]
pub struct EmptyJson{

}

#[derive(Serialize, Deserialize)]
pub struct Status {
    pub status: String
}

pub const API_BASE: &str = "/api";
pub const PORTAL_BASE: &str = "";

pub struct Service {}
impl Service {
    pub fn endpoint (url: &str, domain: usize) -> String {
        let path = match url.strip_prefix(PORTAL_BASE) {
            Some(p) => {
                p
            },
            None => {
                url
            }
        };

        format!("{}/{}{}",API_BASE,domain, path)
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

    pub async fn post<T: Serialize + DeserializeOwned>(path: &str, domain: usize, data: T)
    -> Result<EmptyJson, Error>{
        let endpoint = Self::endpoint(path, domain);
        let request = Request::post(&endpoint)
            .json(&data)?;

        let response = request
            .send()
            .await?;

        response
            .json()
            .await
    }

    pub async fn patch<T: Serialize + DeserializeOwned>(path: &str, domain: usize, data: T)
    -> Result<EmptyJson, Error>{
        let endpoint = Self::endpoint(path, domain);
        let request = Request::patch(&endpoint)
            .json(&data)?;
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
