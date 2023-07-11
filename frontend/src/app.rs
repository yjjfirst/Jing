use yew::prelude::*;
use yew_router::prelude::*;
use super::components::sidebar::{SideBar};
use crate::pages::ringing_group::{RingingGroupsRoute, ringinggroups_switch};
use crate::components::cards::{Cards};

#[derive(Clone, Debug, PartialEq)]
pub struct Env {
    pub base_url: String,
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Cards,
    #[at("/ringing-group")]
    RingingGroupsRoot,
    #[at("/ringing-group/*")]
    RingingGroups,

}

#[function_component(App)]
pub fn app() -> Html {
    let ctx = use_state (|| Env {
        base_url: "http://teleman.me:9090/api".to_owned(),
    });

    html! {
        <ContextProvider<Env> context={(*ctx).clone()}>
            <BrowserRouter>
                <div class="flex">
                    <SideBar></SideBar>
                    <div class="grow ml-4 mr-1">
                        <Switch<Route> render={switch} />
                    </div>                    
                </div>
            </BrowserRouter>
        </ContextProvider<Env>>
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