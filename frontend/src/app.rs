use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;
use crate::store::Store;
use super::components::sidebar::{SideBar};
use crate::pages::ringing_group::{RingingGroupsRoute, ringinggroups_switch};
use crate::components::cards::{Cards};
use crate::components::alert::{AlertComponent, Props as AlertProps};
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
    let (store, _) = use_store::<Store>();
    let message = store.alert_input.alert_message.clone();
    let show_alert: bool = store.alert_input.show_alert;
    
    let alert_props = AlertProps {
        message,
        delay_ms: 5000,
    };    
    let ctx = use_state (|| Env {
        base_url: "http://teleman.me:9090/api".to_owned(),
    });

    html! {
        <ContextProvider<Env> context={(*ctx).clone()}>
            <BrowserRouter>
                if show_alert {
                    <AlertComponent
                        message={alert_props.message}
                        delay_ms={alert_props.delay_ms}
                    />
                }
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