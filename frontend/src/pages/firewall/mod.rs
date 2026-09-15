pub mod model;

use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::components::header::Header;
use crate::components::toggle::Toggle;
use model::FirewallRule;
use crate::models::Service;
use crate::store::Store;
use crate::store::{alert_error, alert_info};

#[derive(Clone, Routable, PartialEq)]
pub enum FirewallRoute {
    #[at("/firewall")]
    Index,
}

#[function_component]
pub fn FirewallList() -> Html {
    let rules = use_state(Vec::<FirewallRule>::new);
    let (store, dispatch) = use_store::<Store>();
    let loc = use_location().unwrap();

    {
        let rules = rules.clone();
        let store = store.clone();
        use_effect_with((), move |_| {
            let store = store.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match Service::index(loc.path(), store.selected_domain_id).await {
                    Ok(fetched_rules) => rules.set(fetched_rules),
                    Err(_) => {}
                }
            });
        });
    }
    
    let handle_changed = {
        let store = store.clone();
        let dispatch = dispatch.clone();
        
        move |id: i32,checked: bool| {
           let store = store.clone();
           let dispatch = dispatch.clone();
           wasm_bindgen_futures::spawn_local(async move {
               match Service::post(&format!("/firewall/{}",id), store.selected_domain_id, checked).await {
                   Ok(_) => {
                       alert_info("Firewall rule updated successfully.".to_string(), dispatch.clone());
                   }
                   Err(_) => {
                       alert_error("Failed to update firewall rule.".to_string(), dispatch.clone());
                   }
               }
           });
       }
    };

    html! {
        <div class="grow mr-2">
            <Header title="System -> Firewall"></Header>
            <div class="divider my-1"></div>
            <table class="table table-zebra">
                <thead>
                    <tr>
                        <th>{"IP Address"}</th>
                        <th>{"Action"}</th>
                        <th>{"Created At"}</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        rules.iter().map(|rule| {
                            html! {
                                <tr>
                                    <td>{rule.ip_address.clone()}</td>
                                    <td>
                                        <Toggle onchange = {
                                            let handle_changed = handle_changed.clone();
                                            let id = rule.id;
                                            Callback::from(move| checked |{
                                                let handle_changed = handle_changed.clone();
                                                handle_changed(id, checked);
                                            })
                                        } 
                                        checked={rule.action == "deny"} />
                                    </td>
                                    <td>{rule.created_at.format("%Y-%m-%d %H:%M:%S").to_string()}</td>
                                </tr>
                            }
                        }).collect::<Html>()
                    }
                </tbody>
            </table>
        </div>
    }
}

pub fn firewall_switch(route: FirewallRoute) -> Html {
    match route {
        FirewallRoute::Index => html! { <FirewallList /> },
    }
}
