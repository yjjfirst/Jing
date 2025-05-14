use std::mem::swap;

use wasm_bindgen::JsCast;
use web_sys::{FormData, SubmitEvent, HtmlDialogElement};
use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yew_icons::{Icon, IconId};

use crate::models::callcenter::queue::Queue;
use crate::store::{alert_info, alert_error, Store};
use crate::models::Service;

#[derive(Clone, Routable, PartialEq)]
pub enum QueueRoute {
    #[at("/callcenter/queue")]
    Index,
}

#[function_component]
pub fn QueueList() -> Html {
    let loc = use_location().unwrap().clone();
    let (store,_) = use_store::<Store>();
    let queues: UseStateHandle<Vec<Queue>> = use_state(||vec![]);
    {
        let queues = queues.clone();
        use_effect_with((), move|_| {
            let queues = queues.clone();
            let url = format!("{}", loc.path());
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_queues: Vec<Queue> =
                    Service::index(&url, store.selected_domain.clone())
                        .await
                        .unwrap();
                queues.set(fetched_queues);
            });
        });
    }

    html!{
        <table class="table table-zebra">
            <thead>
                <tr>
                    <th>{"Extension"}</th>
                    <th>{"Name"}</th>
                </tr>
            </thead>
            <tbody>
            {
                queues.iter().map(|q|{
                    html!{
                        <tr>
                            <td>{q.exten.clone()}</td>
                            <td>{q.name.clone()}</td>
                            <td>
                                <div class="btn btn-square btn-outline btn-sm">
                                    <Icon icon_id={IconId::LucideEdit}/>   
                                </div>
                                <div class="btn btn-square btn-outline btn-sm">
                                    <Icon icon_id={IconId::LucideTrash}/>   
                                </div>
                            </td>
                        </tr>
                    }
                }).collect::<Vec<Html>>()
            }
            </tbody>
        </table>        
    }
}


pub fn queue_switch(route: QueueRoute) -> Html {
    match route {
        QueueRoute::Index => html!{<QueueList />},
    }
}
