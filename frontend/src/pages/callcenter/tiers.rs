use wasm_bindgen::JsCast;
use web_sys::{FormData, SubmitEvent, HtmlDialogElement};
use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Clone, Routable, PartialEq)]
pub enum TierRoute {
    #[at("/callcenter/tier")]
    Index,
}

#[function_component]
pub fn Tiers() -> Html {
    html!{
        <div>{"Tiers"}</div>
    }
}

pub fn tier_switch(route: TierRoute) -> Html {
    match route {
        TierRoute::Index => html!{<Tiers />},
    }
}
