use yew::prelude::*;
use serde::{Serialize, Deserialize};
use yewdux::prelude::*;
use yew_hooks::use_interval;
use wasm_bindgen_futures::spawn_local;
use yew_icons::{Icon, IconData};

use crate::store::{Store};
use crate::models::Service;

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub direction: String,
    pub cid_name: String,
    pub cid_num: String,
    pub callee_num: String
}

#[component]
pub fn ActiveCallCard() -> Html {
    let channels: UseStateHandle<Vec<Channel>> = use_state(||vec![]);
    let (store, _) = use_store::<Store>();
    {
        let channels = channels.clone();
        let store = store.clone();
        use_effect_with(store.selected_domain_id, move |_| {
            let channels = channels.clone();
            let store = store.clone();
            spawn_local(async move{
                let channels_fetched: Vec<Channel> = Service::index("/channel", store.selected_domain_id)
                    .await
                    .unwrap();
                channels.set(channels_fetched);
            });
        });
    }

    {
        let channels = channels.clone();
        let store = store.clone();
        use_interval(move ||{
            let channels = channels.clone();
            let store = store.clone();
            spawn_local(async move{
                let channels_fetched: Vec<Channel> = Service::index("/channel", store.selected_domain_id)
                    .await
                    .unwrap();
                channels.set(channels_fetched);
            });
        }, 5 * 1000);
    }

    html! {
        <div class="overflow-x-auto pbx-card">
            <table class="table table-sm">
                <thead>
                    <tr>
                        <th>{"Direction"}</th>
                        <th>{"CID name"}</th>
                        <th>{"CID number"}</th>
                        <th>{"Callee Number"}</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        channels.iter().map(|ch|{
                            html!{
                                <tr>
                                    {
                                        if ch.direction == "inbound" {
                                            html!{
                                               <td>
                                                   <Icon class="mr-1"
                                                       data={IconData::LUCIDE_ARROW_DOWN_RIGHT}
                                                       style={"color: green;"}
                                                   />
                                               </td>
                                            }
                                        } else {
                                            html!{
                                                <td>
                                                    <Icon class="mr-1"
                                                        data={IconData::LUCIDE_ARROW_UP_LEFT}
                                                        style={"color: yellow;"}
                                                    />
                                                </td>
                                            }
                                        }
                                    }
                                    <td>{ch.cid_name.clone()}</td>
                                    <td>{ch.cid_num.clone()}</td>
                                    <td>{ch.callee_num.clone()}</td>
                                </tr>
                            }}
                        ).collect::<Html>()
                    }
                </tbody>
            </table>
        </div>
    }
}