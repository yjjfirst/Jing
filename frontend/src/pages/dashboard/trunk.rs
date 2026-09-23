use yew::prelude::*;
use yew_hooks::use_interval;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use crate::models::Service;
use yewdux::prelude::*;
use yew_icons::{Icon, IconData};
use compound_duration::format_dhms;

use crate::store::{Store};
use crate::pages::gateway::model::{Gateway};

#[component]
pub fn TrunkCard() -> Html {
    let gateways: UseStateHandle<Vec<Gateway>> = use_state(||vec![]);
    let (store, _) = use_store::<Store>();
    {
        let gateways = gateways.clone();
        let store = store.clone();
        use_effect_with(store.selected_domain_id, move |_| {
            let gateways = gateways.clone();
            let store = store.clone();
            spawn_local(async move{
                let gateways_fetched: Vec<Gateway> = Service::index("/gateway", store.selected_domain_id)
                    .await
                    .unwrap();
                gateways.set(gateways_fetched);
            });
        });
    }

    {
        let gateways = gateways.clone();
        let store = store.clone();
        use_interval(move ||{
            let gateways = gateways.clone();
            let store = store.clone();
            spawn_local(async move{
                let gateways_fetched: Vec<Gateway> = Service::index("/gateway", store.selected_domain_id)
                    .await
                    .unwrap();
                gateways.set(gateways_fetched);
            });
        }, 10 * 1000);
    }

    html!{
        <div class="overflow-x-auto">
            <table class="table table-sm">
                <thead>
                    <tr>
                        <th></th>
                        <th>{"Name"}</th>
                        <th>{"Status"}</th>
                        <th>{"Up Time"}</th>
                        <th>{"Inbound (F/T)"}</th>
                        <th>{"Outbound (F/T)"}</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        gateways.iter().map(|g|{
                            html!{
                                <tr>
                                    {
                                        if g.status.status == "UP" {
                                            html!{
                                               <td>
                                                   <Icon class="mr-1"
                                                       data={IconData::LUCIDE_CHECK_CIRCLE_2}
                                                       style={"color: green;"}
                                                   />
                                               </td>
                                            }
                                        } else {
                                            html!{
                                                <td>
                                                    <Icon class="mr-1"
                                                        data={IconData::LUCIDE_X_CIRCLE}
                                                        style={"color: gray;"}
                                                    />
                                                </td>
                                            }
                                        }
                                    }
                                    <td>{g.gateway_name.clone()}</td>
                                    <td>{g.status.status.clone()}</td>
                                    <td>{format_dhms(g.status.up_seconds)}</td>
                                    <td>{format!("{}/{}", g.status.in_failed, g.status.in_total)}</td>
                                    <td>{format!("{}/{}", g.status.out_failed, g.status.out_total)}</td>
                                </tr>
                            }}
                        ).collect::<Html>()
                    }
                </tbody>
            </table>
        </div>
    }
}