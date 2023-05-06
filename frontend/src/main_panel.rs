use yew::prelude::*;
use yew_router::prelude::*;
use yew::{Properties};
use web_sys::{MouseEvent};
use gloo_dialogs::{alert};
use crate::ringing_group::{RingingGroups};
use crate::cards::{Cards};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Cards,
    #[at("/ringing-group")]
    RingingGroups
}

#[derive(Properties, PartialEq)]
pub struct Props {
    onclick: Callback<MouseEvent>,
}


#[function_component]
pub fn MainPanel() -> Html {
    let onclick = Callback::from(|_e: MouseEvent| {
        let greeting = String::from("Hi there");
        alert(&greeting);
    });

    html! {
        <div class="grow ml-4 mr-1">
            <h1 class="p-4 border-b-zinc-400 border-b mb-4">{"Title"}</h1>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </div>
    }
}


fn switch(routes: Route) -> Html {
    match routes {
        Route::Cards => html! {
            <Cards />
        },
        Route::RingingGroups => html! {
            <RingingGroups />
        }
    }
}
