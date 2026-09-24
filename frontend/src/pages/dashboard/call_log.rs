use yew::prelude::*;
use yewdux::prelude::*;
use yew_hooks::use_interval;
use wasm_bindgen_futures::spawn_local;

use crate::store::{Store};
use crate::models::Service;
use crate::pages::cdr::model::Cdr;

#[component]
pub fn CallLogCard() -> Html {
    let (store,_) = use_store::<Store>();
    let cdrs: UseStateHandle<Vec<Cdr>> = use_state(||vec![]);
    {
        let cdrs = cdrs.clone();
        let store = store.clone();
        use_effect_with((), move|_|{
            let store = store.clone();
            let cdrs = cdrs.clone();
            spawn_local(async move {
                let fetched_cdrs: Vec<Cdr> =
                    Service::index("/cdr", store.selected_domain_id.clone())
                        .await
                        .unwrap();
                cdrs.set(fetched_cdrs.into_iter().take(5).collect());
            });
        });
    }
    {
        let cdrs = cdrs.clone();
        let store = store.clone();
        use_interval(move || {
            let store = store.clone();
            let cdrs = cdrs.clone();
            spawn_local(async move {
                let fetched_cdrs: Vec<Cdr> =
                    Service::index("/cdr", store.selected_domain_id.clone())
                        .await
                        .unwrap();
                cdrs.set(fetched_cdrs.into_iter().take(5).collect());
            });
        }, 10 * 1000);
    }

    html! {
        <div class="overflow-x-auto pbx-card">
            <table class="table table-sm">
                <thead>
                    <tr>
                        <th>{"CID Number"}</th>
                        <th>{"CID name"}</th>
                        <th>{"Callee Number"}</th>
                        <th>{"Duration"}</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        cdrs.iter().map(|c|{
                            html!{
                                <tr>
                                    <td>{c.caller_id_number.clone()}</td>
                                    <td>{c.caller_id_name.clone()}</td>
                                    <td>{c.destination_number.clone()}</td>
                                    <td>{c.duration}</td>
                                </tr>
                            }}
                        ).collect::<Html>()
                    }
                </tbody>
            </table>
        </div>
    }
}