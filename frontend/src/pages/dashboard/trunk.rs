use yew::prelude::*;
use yew_hooks::use_interval;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use crate::models::Service;
use yewdux::prelude::*;
use crate::store::{Store};

#[component]
pub fn TrunkCard() -> Html {
    html!{
        <div>{"Trunk card"}</div>
    }
}
