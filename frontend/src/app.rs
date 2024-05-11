use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;
use crate::store::{Store, select_domain, set_domains};
use super::components::sidebar::SideBar;
use crate::pages::ringing_group::{RingingGroupsRoute, ringinggroups_switch};
use crate::pages::extension::{ExtensionsRoute, extensions_switch};
use crate::components::dashboard::Dashboard;
use crate::components::alert::{AlertType, AlertComponent, Props as AlertProps};
use crate::components::banner::Banner;
use crate::services::domain::Domain;

#[derive(Clone, Debug, PartialEq)]
pub struct Env {
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Dashboard,
    #[at("/ringing-group")]
    RingingGroupsRoot,
    #[at("/ringing-group/*")]
    RingingGroups,
    #[at("/extension")]
    ExtensionRoot,
    #[at("/extension/*")]
    Extensions,
}

#[function_component(App)]
pub fn app() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let message = store.alert_input.alert_message.clone();
    let show_alert: bool = store.alert_input.show_alert;
    
    let alert_props = AlertProps {
        message,
        delay_ms: 5000,
        alert_type: AlertType::INFO
    };    
    let ctx = use_state (|| Env {
    });

    use_effect_with((), move |_| {
        let disp = dispatch.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let domains = Domain::index().await;
            select_domain(domains.first().unwrap().id, dispatch);
            set_domains(domains, disp);                
        })
    });

    html! {
        <ContextProvider<Env> context={(*ctx).clone()}>
            <BrowserRouter>
                if show_alert {
                    <AlertComponent
                        message={alert_props.message}
                        delay_ms={alert_props.delay_ms}
                        alert_type={alert_props.alert_type}
                    />
                }
                if store.selected_domain != 0 {
                    <div class="flex flex-col">
                        <Banner></Banner>
                        <div class="flex grow ml-4 mr-1">
                            <SideBar></SideBar>
                            <div class="grow ml-4 mr-1">
                                <Switch<Route> render={switch} />    
                            </div>
                        </div>                    
                    </div>
                }
            </BrowserRouter>
        </ContextProvider<Env>>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Dashboard => html! {
            <Dashboard />
        },
        Route::RingingGroupsRoot => html! {
            <Switch<RingingGroupsRoute> render={ringinggroups_switch}/>
        },
        Route::RingingGroups => html! {
            <Switch<RingingGroupsRoute> render={ringinggroups_switch}/>
        },
        Route::ExtensionRoot => html! {
            <Switch<ExtensionsRoute> render={extensions_switch}/>
        },
        Route::Extensions => html! {
            <Switch<ExtensionsRoute> render={extensions_switch}/>
        }
    }
}