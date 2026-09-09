pub mod model;

use gloo_net::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::header::Header;
use crate::models::API_BASE;
use model::FirewallRule;

#[derive(Clone, Routable, PartialEq)]
pub enum FirewallRoute {
    #[at("/firewall")]
    Index,
}

#[function_component]
pub fn FirewallList() -> Html {
    let rules = use_state(Vec::<FirewallRule>::new);
    let error = use_state(|| None::<String>);
    let rules_for_effect = rules.clone();
    let error_for_effect = error.clone();

    use_effect_with((), move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let response = match Request::get(&format!("{}/firewall", API_BASE)).send().await {
                Ok(response) if response.ok() => response,
                Ok(response) => {
                    error_for_effect.set(Some(format!(
                        "Failed to load firewall rules (HTTP {})",
                        response.status()
                    )));
                    return;
                }
                Err(err) => {
                    error_for_effect.set(Some(format!(
                        "Failed to load firewall rules: {}",
                        err
                    )));
                    return;
                }
            };

            match response.json::<Vec<FirewallRule>>().await {
                Ok(fetched_rules) => rules_for_effect.set(fetched_rules),
                Err(err) => error_for_effect.set(Some(format!(
                    "Failed to decode firewall rules: {}",
                    err
                ))),
            }
        });
    });

    html! {
        <div class="grow mr-2">
            <Header title="System -> Firewall"></Header>
            <div class="divider my-1"></div>
            if let Some(message) = (*error).clone() {
                <div class="alert alert-error">{message}</div>
            }
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
                                    <td>{rule.action.clone()}</td>
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
