use yew::prelude::*;
use yew_hooks::use_interval;
use wasm_bindgen_futures::spawn_local;
use crate::models::Service;
use yewdux::prelude::*;
use yew_icons::{Icon, IconData};
use crate::store::{Store};

use crate::pages::user::model::User;

#[component]
pub fn ExtenCard() -> Html {
    let users: UseStateHandle<Vec<User>> = use_state(||vec![]);
    let (store, _) = use_store::<Store>();
    {
        let users = users.clone();
        let store = store.clone();
        use_effect_with(store.selected_domain_id, move |_| {
            let users = users.clone();
            let store = store.clone();
            spawn_local(async move{
                let users_fetched: Vec<User> = Service::index("/user", store.selected_domain_id)
                    .await
                    .unwrap();
                users.set(users_fetched);
            });
        });
    }

    {
        let users = users.clone();
        let store = store.clone();
        use_interval(move ||{
            let users = users.clone();
            let store = store.clone();
            spawn_local(async move{
                let users_fetched: Vec<User> = Service::index("/user", store.selected_domain_id)
                    .await
                    .unwrap();
                users.set(users_fetched);
            });
        }, 10 * 1000);
    }

    html!{
        <div class="overflow-x-auto pbx-card">
            <table class="table table-sm">
                <thead>
                    <tr>
                        <th></th>
                        <th>{"Extension"}</th>
                        <th>{"IP Address"}</th>
                        <th>{"User Agent"}</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        users.iter().map(|u|{
                            html!{
                                <tr>
                                    {
                                        if u.status.get("ping").map_or("",|v|v).to_string() == "Reachable" {
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
                                    <td>{u.user_id.clone()}</td>
                                    <td>{u.status.get("ip_addr").map_or("",|v|v).to_string()}</td>
                                    <td>{u.status.get("agent").map_or("",|v|v).to_string()}</td>
                                </tr>
                            }}
                        ).collect::<Html>()
                    }
                </tbody>
            </table>
        </div>
    }
}