use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

use gloo_net::http::Request;

use crate::store::{Store, select_domain, set_domains, set_is_authenticated, set_username};
use super::components::sidebar::SideBar;
use crate::pages::ring_group::{RingGroupsRoute, ringgroups_switch};
use crate::pages::user::{UserRoute, user_switch};
use crate::pages::gateway::{GatewayRoute, gateway_switch};
use crate::pages::route_out::{OutboundRoute, outbound_switch};
use crate::pages::route_in::{InboundRoute, inbound_switch};
use crate::pages::cdr::{CdrRoute, cdr_switch};
use crate::pages::dashboard::Dashboard;
use crate::pages::sound_file::{SoundFileRoute, sound_file_switch};
use crate::pages::sound::{SoundRoute, sound_switch};
use crate::pages::conference::{ConfRoute, conf_switch};
use crate::pages::ivr::{IvrRoute, ivr_switch};
use crate::pages::callcenter::{CallcenterRootRoute, callcenter_root_switch};
use crate::pages::login::Login;
use crate::components::alert::{AlertType, AlertComponent, Props as AlertProps};
use crate::components::banner::Banner;
use crate::models::domain::Domain;
use crate::models::{API_BASE, Status};

#[derive(Clone, Debug, PartialEq)]
pub struct Env {
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Dashboard,
    #[at("/ring-group")]
    RingGroupsRoot,
    #[at("/ring-group/*")]
    RingGroups,
    #[at("/user")]
    ExtensionRoot,
    #[at("/user/*")]
    Extensions,
    #[at("/gateway")]
    GatewayRoot,
    #[at("/gateway/*")]
    Gateways,
    #[at("/outbound")]
    OutboundRoot,
    #[at("/outbound/*")]
    Outbounds,
    #[at("/inbound")]
    InboundRoot,
    #[at("/inbound/*")]
    Inbounds,
    #[at("/cdr")]
    Cdr,
    #[at("/sound-file")]
    SoundFileRoot,
    #[at("/sound-file/*")]
    SoundFile,
    #[at("/sound")]
    SoundRoot,
    #[at("/sound/*")]
    Sound,
    #[at("/ivr")]
    IvrRoot,
    #[at("/ivr/*")]
    Ivr,
    #[at("/conference")]
    ConferenceRoot,
    #[at("/conference/*")]
    Conference,    
    #[at("/callcenter/queue")]
    CallcenterRoot,
    #[at("/callcenter/*")]
    Callcenter,    
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

    {
        let dispatch = dispatch.clone();
        use_effect_with((),move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("{}/login/verify", API_BASE);
                let response = Request::get(&url)
                    .send()
                    .await;
                match response {
                    Ok(res) => {
                        if res.ok() {
                            let s: Status = res.json().await.unwrap();
                            set_username(s.status, dispatch.clone());
                            set_is_authenticated(true, dispatch);
                       } else {
                            set_is_authenticated(false, dispatch);
                       }
                    }
                    Err(_) => {
                        set_is_authenticated(false, dispatch);
                    }
                }
            });
        });
    }

    {
        let store = store.clone();
        let dispatch = dispatch.clone();
        use_effect_with(store.is_authenticated, move |_| {
            if store.is_authenticated {
                let dispatch = dispatch.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let domains = Domain::index().await;
                    select_domain(domains.first().unwrap().id, dispatch.clone());
                    set_domains(domains, dispatch.clone());                
                })
            }
        });
    }
    
    html! {
        <ContextProvider<Env> context={(*ctx).clone()}>
            if store.is_authenticated == true {
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
            } else {
                <Login />
            }
        </ContextProvider<Env>>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Dashboard => html! {
            <Dashboard />
        },
        Route::RingGroupsRoot => html! {
            <Switch<RingGroupsRoute> render={ringgroups_switch}/>
        },
        Route::RingGroups => html! {
            <Switch<RingGroupsRoute> render={ringgroups_switch}/>
        },
        Route::ExtensionRoot => html! {
            <Switch<UserRoute> render={user_switch}/>
        },
        Route::Extensions => html! {
            <Switch<UserRoute> render={user_switch}/>
        },
        Route::GatewayRoot => html! {
            <Switch<GatewayRoute> render={gateway_switch} />
        },
        Route::Gateways => html! {
            <Switch<GatewayRoute> render={gateway_switch} />
        },
        Route::OutboundRoot => html! {
            <Switch<OutboundRoute> render={outbound_switch} />
        },
        Route::Outbounds => html! {
            <Switch<OutboundRoute> render={outbound_switch} />
        },
        Route::InboundRoot => html! {
            <Switch<InboundRoute> render={inbound_switch} />
        },        
        Route::Inbounds => html! {
            <Switch<InboundRoute> render={inbound_switch} />
        },
        Route::Cdr => html! {
            <Switch<CdrRoute> render={cdr_switch} />
        },
        Route::SoundFileRoot => html! {
            <Switch<SoundFileRoute> render={sound_file_switch} />
        },
        Route::SoundFile => html! {
            <Switch<SoundFileRoute> render={sound_file_switch} />
        },
        Route::ConferenceRoot => html! {
            <Switch<ConfRoute> render={conf_switch}/>
        },
        Route::Conference => html! {
            <Switch<ConfRoute> render={conf_switch}/>
        },
        Route::SoundRoot => html! {
            <Switch<SoundRoute> render={sound_switch} />
        },
        Route::Sound => html! {
            <Switch<SoundRoute> render={sound_switch} />
        },
        Route::IvrRoot => html! {
            <Switch<IvrRoute> render={ivr_switch} />
        },
        Route::Ivr => html! {
            <Switch<IvrRoute> render={ivr_switch} />
        },
        Route::CallcenterRoot => html! {
            <Switch<CallcenterRootRoute> render={callcenter_root_switch} />
        },
        Route::Callcenter => html! {
            <Switch<CallcenterRootRoute> render={callcenter_root_switch} />
        },        
    }
}