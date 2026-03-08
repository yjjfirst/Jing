use gloo_net::http::Request;
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};
use yew::prelude::*;
use serde::{Deserialize, Serialize};
use yewdux::prelude::use_store;

use crate::store::{Store, set_is_authenticated};
use crate::models::API_BASE;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[function_component]
pub fn Login() -> Html {
    let (_, dispatch) = use_store::<Store>();

    let handle_submit = {
        let dispatch = dispatch.clone();
        Callback::from(move|e: SubmitEvent| {
            let dispatch = dispatch.clone();
            e.prevent_default();
            let form_data = FormData::new_with_form(
                &e.target().unwrap().dyn_into::<HtmlFormElement>().unwrap()).unwrap();
            
            let username = form_data
                .get("username")
                .as_string()
                .unwrap();

            let password = form_data
                .get("password")
                .as_string()
                .unwrap();
            let data = LoginData {
                username, password
            };

            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("{}/login", API_BASE);
                let req = Request::post(&url)
                    .json(&data).unwrap();
                    
                let res = req
                    .send()
                    .await
                    .unwrap();
                if res.ok() {
                    set_is_authenticated(true, dispatch);
                }
            });
        })
    };

    html! {
        <div class="relative flex flex-col justify-center h-screen overflow-hidden">
            <div class="w-full p-6 m-auto rounded-md shadow-md ring-2 ring-gray-800/50 lg:max-w-lg">
                <h1 class="text-4xl font-semibold text-center text-gray-700">{"PBX"}</h1>
                <form class="space-y-4" onsubmit={handle_submit}>
                    <div>
                        <label class="label">
                            <span class="text-base label-text">{"User Name"}</span>
                        </label>
                        <input type="text" 
                            placeholder="User Name" 
                            class="w-full input input-bordered" 
                            name="username"/>
                    </div>
                    <div>
                        <label class="label">
                            <span class="text-base label-text">{"Password"}</span>
                        </label>
                        <input type="password" 
                            placeholder="Password" 
                            class="w-full input input-bordered" 
                            name="password"/>
                    </div>
                    <div>
                        <button class="btn-neutral btn btn-block">{"Login"}</button>
                    </div>
                </form>
            </div>
        </div>
    }
}