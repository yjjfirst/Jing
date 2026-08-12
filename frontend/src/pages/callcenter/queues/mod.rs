pub mod model;

use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::{FormData, SubmitEvent, HtmlDialogElement};
use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yew_icons::{Icon, IconData};

use model::{Queue, QueueParam};
use crate::store::{alert_info, alert_error, Store};
use crate::models::Service;

use crate::components::header::Header;
use crate::components::label::Label;
use crate::components::dialog::Dialog;
use crate::components::input::Input;
use crate::components::select::Select;
use crate::components::action_buttons::ActionButtons;

#[derive(Clone, Routable, PartialEq)]
pub enum QueueRoute {
    #[at("/callcenter/queue")]
    Index,
    #[at("/callcenter/queue/:id")]
    Get {id: i32}
}

#[derive(Clone, PartialEq, Properties)] 
pub struct QueueDetailsProps {
    #[prop_or(0)]
    pub id: i32,
}

#[derive(Clone, PartialEq, Properties)]
pub struct QueueListItemProps {
    pub id: i32,
    pub exten: String,
    pub name: String,
    pub ondel: Callback<i32>
}

#[function_component]
pub fn QueueListItem(props: &QueueListItemProps) -> Html {
    let nav = use_navigator().unwrap();
    let dialog_ref: NodeRef = use_node_ref(); 
    let loc: Location = use_location().unwrap();
    let (store,_) = use_store::<Store>();
    let ondel = props.ondel.clone();

    let id = props.id;

    let onconfirm: Callback<bool> = Callback::from(move|_e: bool|{  
        let loc = loc.clone();
        let store = store.clone();
        let ondel = ondel.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let path = format!("{}/{}", loc.path(), id);
            Service::delete(&path, store.clone().selected_domain_id)
                .await
                .unwrap();
            ondel.emit(id);
        });    
    });

    let handle_del: Callback<MouseEvent> = {
        let dialog_ref = dialog_ref.clone();
        Callback::from(move |_e| {
            let d = dialog_ref.cast::<HtmlDialogElement>().unwrap();
                d.show_modal().unwrap();
        })
    };

    html!{
        <tr>
            <td>{props.exten.clone()}</td>
            <td>{props.name.clone()}</td>
            <td>
                <div class="flex justify-end">
                    <div 
                        onclick={
                            let nav = nav.clone();
                            Callback::from(move |_e: MouseEvent|{
                                nav.push(&QueueRoute::Get {id: id});
                            })
                        } 
                        class="btn btn-square btn-outline btn-sm mr-1">
                        <Icon data={IconData::LUCIDE_EDIT}/>   
                    </div>
                    <div
                        onclick={handle_del}
                        class="btn btn-square btn-outline btn-sm">
                        <Icon data={IconData::LUCIDE_TRASH}/>   
                    </div>
                </div>
            </td>
            <Dialog
                d_ref = {dialog_ref}
                title={"Warning!"} 
                contents={format!("Are you sure to delete the Queue: {}?", props.exten.clone())}
                {onconfirm}
                >
            </Dialog>              
        </tr>
    }
}

#[function_component]
pub fn QueueList() -> Html {
    let nav = use_navigator().unwrap();
    let (store,_) = use_store::<Store>();
    let queues: UseStateHandle<Vec<Queue>> = use_state(||vec![]);
        
    {
        let store = store.clone();
        let queues = queues.clone();
        use_effect_with((), move|_| {
            let queues = queues.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_queues: Vec<Queue> =
                    Queue::list(store.selected_domain_id).await;
                queues.set(fetched_queues);
            });
        });
    }

    let handle_add: Callback<MouseEvent> = {
        let nav = nav.clone();
        Callback::from(move|_e: MouseEvent|{
            nav.push(&QueueRoute::Get {id: 0});        
        })
    };

    let handle_del: Callback<i32>  = {
        let queues = queues.clone();
        Callback::from(move | id: i32| {
            let filtered: Vec<Queue> = queues
                .iter()
                .filter(|q|q.id != id)
                .map(|s|s.clone())
                .collect();
            queues.set(filtered);
        })
    };

    html!{
    <div class="grow mr-2">
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
                        <QueueListItem
                            id={q.id}
                            exten={q.exten.clone()}
                            name={q.name.clone()}
                            ondel={handle_del.clone()}
                        >
                        </QueueListItem>
                    }
                }).collect::<Vec<Html>>()
            }
            </tbody>
        </table>
        <div class="flex flex-row-reverse pr-4">
            <div onclick={handle_add} class="btn btn-square btn-outline btn-sm" >
                <Icon data={IconData::LUCIDE_PLUS}/>   
            </div>
        </div>         
    </div>      
    }
}

#[function_component]
pub fn QueueDetails(props: &QueueDetailsProps) -> Html {
    let loc = use_location().unwrap();
    let nav = use_navigator().unwrap();
    let(store, dispatch) = use_store::<Store>();
    let queue = use_state(||Queue::new());

    {
        let store = store.clone();
        let queue = queue.clone();
        let loc = loc.clone();

        use_effect_with((), move |_| {
            let queue = queue.clone();
            let loc = loc.clone();
            let store = store.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_queue = 
                    Service::get(loc.path(), store.selected_domain_id)
                        .await
                        .unwrap();
                queue.set(fetched_queue);
            });
        });
    }

    let handle_cancel = {
        let nav = nav.clone();
        Callback::from(move|_| {
            nav.push(&QueueRoute::Index);
        })
    };

    let handle_submit = {
        let queue = queue.clone();
        let store = store.clone();
        let dispatch = dispatch.clone();
        let loc = loc.clone();
        let nav = nav.clone();

        Callback::from(move|event: SubmitEvent| {
            let target = event.target().unwrap();
            let form = target.dyn_into().unwrap();            
            let form_data = FormData::new_with_form(&form).unwrap();
            let dispatch = dispatch.clone();
            let loc = loc.clone();
            let nav = nav.clone();
            let store = store.clone();
            let id = queue.id;

            let params = queue.params
                .clone()
                .into_iter()
                .map(|p| {
                    let key = p.0;
                    let mut param = p.1;
                    param.value = form_data.get(&key).as_string().unwrap();
                    (key, param)
                })
                .collect::<HashMap<String, QueueParam>>();
            
            let data = Queue {
                id: queue.id,
                domain_id: queue.domain_id,
                exten: form_data.get("extension").as_string().unwrap(),
                name: form_data.get("name").as_string().unwrap(),
                params: params
            };

            wasm_bindgen_futures::spawn_local(async move {
                let dispatch = dispatch.clone();
                match Service::post(loc.path(), store.selected_domain_id, data).await {
                    Ok(_) => {
                        if id != 0 {
                            alert_info("Update Queue successfully.".to_string(), dispatch);
                        } else {
                            alert_info("Add Queue successfully".to_string(), dispatch);
                        }
                    }
                    Err(_) => {
                        if id != 0 {
                            alert_error("Update Queue failed.".to_string(), dispatch);
                        } else {
                            alert_error("Add Queue failed.".to_string(), dispatch);
                        }
                    }
                }
                nav.push(&QueueRoute::Index);            
            });

            event.prevent_default();
        })
    };

    html!{
        <div class="grow mt-1">
            <Header title= {format!("Queue: {}", queue.exten.clone())}></Header>
            <div class="divider my-1"></div>         
            <form class="w-full" onsubmit={handle_submit}>
                <div class="grid grid-cols-3 gap-1">
                    <Label hidden = {props.id != 0}>{"Extension"}</Label>
                    <Input
                        hidden = {props.id != 0}                    
                        value={queue.exten.clone()}
                        id="extension"
                    /> 
                    <Label>{"Name"}</Label>
                    <Input
                        value={queue.name.clone()}
                        id="name"
                    />
                    <Label>{"strategy"}</Label>
                    <Select
                        options={
                            vec![
                                "ring-all".to_string(),
                                "longest-idle-agent".to_string(),
                                "round-robin".to_string(),
                                "top-down".to_string(),
                                "agent-with-least-talk-time".to_string(),
                                "agent-with-fewest-calls".to_string(),
                                "sequentially-by-agent-order".to_string(),
                                "random".to_string(),
                                "ring-progressively".to_string(),
                            ]
                        }
                        selected = {QueueParam::get("strategy", &queue.params)}
                        id="strategy"
                    />
                    <Label>{"moh"}</Label>
                    <Input
                        value = {QueueParam::get("moh-sound", &queue.params)}
                        id="moh-sound"
                    />
                    <Label>{"Time base score"}</Label>
                    <Select
                        options={vec!["system".to_string(), "queue".to_string()]}
                        selected = {QueueParam::get("time-base-score", &queue.params)}
                        id="time-base-score"
                    />
                    <Label>{"Max wait time"}</Label>
                    <Input
                        value = {QueueParam::get("max-wait-time", &queue.params)}
                        id="max-wait-time"
                    />
                    <Label>{"Max wait time with no agent"}</Label>
                    <Input
                        value = {QueueParam::get("max-wait-time-with-no-agent", &queue.params)}
                        id="max-wait-time-with-no-agent"
                    />
                    <Label>{"Max wait time with no agent time reached"}</Label>
                    <Input
                        value = {QueueParam::get("max-wait-time-with-no-agent-time-reached", &queue.params)}
                        id="max-wait-time-with-no-agent-time-reached"
                    />
                    <Label>{"Tier rules apply"}</Label>
                    <Select
                        options={vec!["true".to_string(), "false".to_string()]}
                        selected = {QueueParam::get("tier-rules-apply", &queue.params)}
                        id="tier-rules-apply"
                    />
                    <Label>{"Tier rule wait second"}</Label>
                    <Input
                        value = {QueueParam::get("tier-rule-wait-second", &queue.params)}
                        id="tier-rule-wait-second"
                    /> 
                    <Label>{"Tier rule wait multiply level"}</Label>
                    <Select
                        options={vec!["true".to_string(), "false".to_string()]}                    
                        selected = {QueueParam::get("tier-rule-wait-multiply-level", &queue.params)}
                        id="tier-rule-wait-multiply-level"
                    />
                    <Label>{"Tier rule no agent no wait"}</Label>
                    <Select
                        options={vec!["true".to_string(), "false".to_string()]}                                            
                        selected = {QueueParam::get("tier-rule-no-agent-no-wait", &queue.params)}
                        id="tier-rule-no-agent-no-wait"
                    />
                    <Label>{"Discard abandoned after"}</Label>
                    <Input
                        value = {QueueParam::get("discard-abandoned-after", &queue.params)}
                        id="discard-abandoned-after"
                    /> 
                    <Label>{"Abandoned resume allowed"}</Label>
                    <Select
                        options={vec!["true".to_string(), "false".to_string()]}                                            
                        selected = {QueueParam::get("abandoned-resume-allowed", &queue.params)}
                        id="abandoned-resume-allowed"
                    />
                </div>
                <ActionButtons oncancel={handle_cancel}/>
            </form>
        </div>
    }
}

pub fn queue_switch(route: QueueRoute) -> Html {
    match route {
        QueueRoute::Index => html!{<QueueList />},
        QueueRoute::Get {id } => html!{<QueueDetails id={id} />}
    }
}
