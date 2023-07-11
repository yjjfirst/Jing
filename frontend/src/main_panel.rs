use yew::prelude::*;
use yew_router::prelude::*;
use yew::{Properties};
use web_sys::{MouseEvent};
use gloo_dialogs::{alert};
use crate::pages::ringing_group::{RingingGroupsRoute, ringinggroups_switch};
use crate::cards::{Cards};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Cards,
    #[at("/ringing-group")]
    RingingGroupsRoot,
    #[at("/ringing-group/*")]
    RingingGroups,

}

#[derive(Properties, PartialEq)]
pub struct Props {
    onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn MainPanel() -> Html {
    let _onclick = Callback::from(|_e: MouseEvent| {
        let greeting = String::from("Hi there");
        alert(&greeting);
    });

    html! {
        <div class="grow ml-4 mr-1">
            <Switch<Route> render={switch} />
        </div>
    }
}


fn switch(routes: Route) -> Html {
    match routes {
        Route::Cards => html! {
            <Cards />
        },
        Route::RingingGroupsRoot => html! {
            <Switch<RingingGroupsRoute> render={ringinggroups_switch}/>
        },
        Route::RingingGroups => html! {
            <Switch<RingingGroupsRoute> render={ringinggroups_switch}/>
        }
    }
}
