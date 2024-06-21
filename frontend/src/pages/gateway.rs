use std::collections::HashMap;
use web_sys::{EventTarget, FormData, SubmitEvent, HtmlFormElement, HtmlDialogElement};
use wasm_bindgen::JsCast;

use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yew_icons::{Icon, IconId};
use crate::store::{show_alert, Store};

use crate::components::header::Header;
use crate::services::Service;
use crate::services::gateway::Gateway;

#[derive(Clone, Routable, PartialEq)]
pub enum GatewayRoute {
    #[at("/gateway")]
    Index,
    #[at("/gateway/:id")]
    Get {id: usize},
}

#[derive(Clone, PartialEq, Properties)] 
pub struct GatewayProps {
    pub gateway: Gateway,
}

#[function_component]
pub fn GatewayDetail() -> Html {
    html! {
        <p>{"Gateway detail"}</p>
    }
}

#[function_component]
pub fn GatewayListItem(props: &GatewayProps) -> Html {
    let gateway = props.gateway.clone();
    html! {
        <tr>
            <th>{gateway.gateway_name.clone()}</th>
            <th>{gateway.proxy.clone()}</th>
            <th>{gateway.register.clone()}</th>
            <th>{gateway.username.clone()}</th>
            <th class="flex justify-end">
                <div class="mr-1">
                    <div class="btn btn-square btn-outline btn-sm">
                        <Icon icon_id={IconId::LucideEdit}/>   
                    </div>
                </div>
                <div>
                    <div class="btn btn-square btn-outline btn-sm">
                        <Icon icon_id={IconId::LucideTrash}/>   
                    </div>
                </div>
            </th>             
        </tr>
    }
}

#[function_component]
pub fn GatewayList() -> Html {
    let loc = use_location().unwrap().clone();    
    let (store,_) = use_store::<Store>();
    let gateways: UseStateHandle<Vec<Gateway>> = use_state(||vec![]);
    let gateways_1 = gateways.clone();
    use_effect_with((), move|_|{
        let gateways = gateways_1.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_gateways: Vec<Gateway> = Service::index(loc.path(), store.selected_domain.clone()).await;
            gateways.set(fetched_gateways);
        });
    });

    let gateways_list: Vec<Html> = gateways.iter().map(|g|{
        html! {
            <GatewayListItem gateway={Gateway {..g.clone()}}></GatewayListItem>
        }
    }).collect();

    html! {
        <div class="grow mr-2">
            <Header title="Connection -> Gateway"></Header>
            <div class="divider my-1"></div>
            <table class="table table-zebra">
                <thead>
                    <tr>
                        <th>{"Name"}</th>
                        <th>{"Proxy"}</th>
                        <th>{"Register"}</th>
                        <th>{"Username"}</th>
                    </tr>
                </thead>
                <tbody>
                {gateways_list}
                </tbody>
            </table>
            <div class="flex flex-row-reverse pr-4">
                <div class="btn btn-square btn-outline btn-sm" >
                    <Icon icon_id={IconId::LucidePlus}/>   
                </div>
            </div>             
        </div>        
    }
}

pub fn gateway_switch(route: GatewayRoute) -> Html {
    match route {
        GatewayRoute::Index => html!{<GatewayList />},
        GatewayRoute::Get { id } => html !{<GatewayDetail />}
    }
}