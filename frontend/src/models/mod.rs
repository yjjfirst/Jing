pub mod domain;
pub mod extension;

use gloo_net::http::Request;
use gloo_net::Error;
use serde::{Serialize, de::DeserializeOwned, Deserialize};
use web_sys::FormData;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{XmlHttpRequest, XmlHttpRequestUpload};
use yewdux::prelude::*;
use yew::prelude::*;
use futures::channel::oneshot;

use crate::store::{Store};

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
        let dispatch = Dispatch::<Store>::global();
        let endpoint = Self::endpoint(path, domain);
        let response = Request::get(&endpoint)
            .send()
            .await?;

        if response.status() == 401 {
            dispatch.reduce_mut(|store| {
                store.is_authenticated = false;
            });
        }

        response
            .json()
            .await
    }

    pub async fn get<T: Serialize + DeserializeOwned>(path: &str, domain: usize) -> Result<T, Error> {
        let dispatch = Dispatch::<Store>::global();
        let endpoint = Self::endpoint(path, domain);
        let response = Request::get(&endpoint)
            .send()
            .await?;

        if response.status() == 401 {
            dispatch.reduce_mut(|store| {
                store.is_authenticated = false;
            });
        }

        let res = response
            .json()
            .await;

        res
    }

    pub async fn post<T: Serialize + DeserializeOwned>(path: &str, domain: usize, data: T)
            -> Result<EmptyJson, Error> {
        let dispatch = Dispatch::<Store>::global();
        let endpoint = Self::endpoint(path, domain);
        let request = Request::post(&endpoint)
            .json(&data)?;

        let response = request
            .send()
            .await?;

        if response.status() == 401 {
            dispatch.reduce_mut(|store| {
                store.is_authenticated = false;
            });
        }

        response
            .json()
            .await
    }

    pub async fn patch<T: Serialize + DeserializeOwned>(path: &str, domain: usize, data: T)
            -> Result<EmptyJson, Error> {
        let dispatch = Dispatch::<Store>::global();
        let endpoint = Self::endpoint(path, domain);
        let request = Request::patch(&endpoint)
            .json(&data)?;
        let response = request
            .send()
            .await?;

        if response.status() == 401 {
            dispatch.reduce_mut(|store| {
                store.is_authenticated = false;
            });
        }

        response
            .json()
            .await
    }

   pub async fn post_with_progress(path: &str,
            domain: usize,
            form_data: &FormData,
            uploading: UseStateHandle<bool>,
            percentage: UseStateHandle<f64>) -> Result<(), JsValue> {
        let url = Self::endpoint(path, domain);
        let xhr = XmlHttpRequest::new()?;
        let (tx, rx) = oneshot::channel::<Result<(), ()>>();

        xhr.open("POST", &url)?;
        let upload: XmlHttpRequestUpload = xhr.upload()?;

        let progress_callback = {
            let percentage = percentage.clone();
            Closure::<dyn FnMut(web_sys::ProgressEvent)>::new(move |e: web_sys::ProgressEvent| {
                let percentage = percentage.clone();
                if e.length_computable() {
                    let loaded = e.loaded();
                    let total = e.total();
                    percentage.set((loaded as f64 / total as f64) * 100.0);
                }
            })
        };

        let mut tx_opt = Some(tx);
        let load_callback = {
            let uploading = uploading.clone();
            Closure::<dyn FnMut(web_sys::Event)>::new(move |_e: web_sys::Event| {
                let uploading = uploading.clone();
                uploading.set(false);
                if let Some(tx) = tx_opt.take() {
                    tx.send(Ok(())).unwrap();
                }
            })
        };

        upload.add_event_listener_with_callback("progress", progress_callback.as_ref().unchecked_ref())?;
        upload.add_event_listener_with_callback("load", load_callback.as_ref().unchecked_ref())?;
        progress_callback.forget();
        load_callback.forget();

        xhr.send_with_opt_form_data(Some(form_data))?;
        let _ = rx.await.unwrap();
        Ok(())
    }

    pub async fn delete(path: &str, domain: usize) -> Result<EmptyJson, Error> {
        let dispatch = Dispatch::<Store>::global();
        let endpoint = Self::endpoint(path, domain);
        let response = Request::delete(&endpoint)
            .send()
            .await?;

        if response.status() == 401 {
            dispatch.reduce_mut(|store| {
                store.is_authenticated = false;
            });
        }

        response
            .json()
            .await
    }
}
