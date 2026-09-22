use yew::prelude::*;
use yew_hooks::use_interval;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use crate::models::Service;
use yewdux::prelude::*;
use crate::store::{Store};

use crate::pages::user::model::User;

#[component]
pub fn ExtenCard() -> Html {
    let (store, _) = use_store::<Store>();
    {
        use_effect_with((), move |_| {
            spawn_local(async move{
                let users_fetched: Vec<User> = Service::index("/user", store.selected_domain_id)
                    .await
                    .unwrap();

            });
        });
    }

    html!{
        <div>{"Extension card"}</div>
    }
}