use wasm_bindgen::JsCast;
use web_sys::{FormData, SubmitEvent, HtmlDialogElement};
use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Clone, Routable, PartialEq)]
pub enum AgentRoute {
    #[at("/callcenter/agent")]
    Index,
}

#[function_component]
pub fn AgentList() -> Html {
    html!{
        <div>{"Agent List"}</div>
    }
}


pub fn agent_switch(route: AgentRoute) -> Html {
    match route {
        AgentRoute::Index => html!{<AgentList />},
    }
}
