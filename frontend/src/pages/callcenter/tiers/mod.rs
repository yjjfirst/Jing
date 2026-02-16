pub mod model;

use web_sys::{FormData, HtmlInputElement, HtmlDialogElement, SubmitEvent};
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yew_icons::{Icon, IconId};

use crate::models::Service;
use model::Tier;
use crate::store::{alert_info, alert_error,Store};
use crate::pages::callcenter::agents::model::Agent;
use crate::pages::callcenter::queues::model::Queue;

use crate::components::input::Input;
use crate::components::label::Label;
use crate::components::select_id::IdSelect;
use crate::components::dialog::Dialog;

#[derive(Clone, Routable, PartialEq)]
pub enum TierRoute {
    #[at("/callcenter/tier")]
    Index,
}

#[derive(Properties, PartialEq)]
pub struct TiersProps {
    pub queue_name: String,
    pub queue_id: i32
}

#[derive(Properties, PartialEq)]
pub struct TierComponentProps {
    pub tier_id: i32,
    pub agent_id: usize,
    pub queue_id: i32,
    pub agent_name: String,
    pub level: i32,
    pub position: i32,
    pub on_update: Callback<i32>
}

#[function_component]
pub fn TierComponent(props: &TierComponentProps) -> Html {
    let level = props.level;
    let pos = props.position;
    let tier_id = props.tier_id;
    let agent_id: usize = props.agent_id;
    let queue_id = props.queue_id;
    let loc = use_location().unwrap();
    let dialog_ref: NodeRef = use_node_ref();
    let onupdate = props.on_update.clone();

    let(store, dispatch) = use_store::<Store>();
    let agents: UseStateHandle<Vec<Agent>> = use_state(||vec![]);

    let options = agents
        .iter()
        .map(|a|a.name.clone())
        .collect::<Vec<String>>();

    let options_id = agents
        .iter()
        .map(|a|a.id)
        .collect::<Vec<usize>>();
    {
        let store = store.clone();
        let agents = agents.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_agents = Agent::list(store.selected_domain).await;
                agents.set(fetched_agents)
            });
        });
    }

    let handle_submit: Callback<SubmitEvent> = {
        let loc = loc.clone();
        let dispatch = dispatch.clone();
        let store = store.clone();

        let onupdate = onupdate.clone();
        Callback::from(move|event: SubmitEvent|{
            let loc = loc.clone();
            let dispatch = dispatch.clone();
            let store = store.clone();
            let onupdate = onupdate.clone();
            event.prevent_default();

            let target = event.target().unwrap();
            let form = target.dyn_into().unwrap();            
            let form_data = FormData::new_with_form(&form).unwrap();

            let agent_id = form_data.get("tier_agent")
                .as_string()
                .unwrap()
                .parse::<i32>()
                .unwrap();

            let level = form_data.get("tier_level")
                .as_string()
                .unwrap()
                .parse::<i32>()
                .unwrap();

            let position = form_data.get("tier_position")
                .as_string()
                .unwrap()
                .parse::<i32>()
                .unwrap();

            let mut agent = Agent::new();
            agent.id = agent_id as usize;
            let tier = Tier {
                id: tier_id,
                queue_id,
                level,
                position,
                agent
            };

            wasm_bindgen_futures::spawn_local(async move {
                let onupdate = onupdate.clone();
                let url = format!("{}/{}", loc.path(), tier.id);
                match Service::post(&url, store.selected_domain, tier).await {
                    Ok(_) => {                        
                        alert_info("Tier updated successfully.".to_string(), dispatch);
                        onupdate.emit(0);
                    },
                    Err(_e) => {
                        alert_error("Tier updated failed.".to_string(), dispatch);
                        onupdate.emit(0);
                    }
                }
            });
        })
    };

    let handle_del: Callback<MouseEvent> = {
        let dialog_ref = dialog_ref.clone();
        Callback::from(move |_e| {
            let d = dialog_ref.cast::<HtmlDialogElement>().unwrap();
                d.show_modal().unwrap();
        })
    };

    let handle_confirm: Callback<bool> = Callback::from(move|_e: bool|{
        let loc = loc.clone();
        let store = store.clone();
        let onupdate = onupdate.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let path = format!("{}/{}", loc.path(), tier_id);
            Service::delete(&path, store.clone().selected_domain)
                .await
                .unwrap();
            onupdate.emit(0);
        });
    });

    html!{
        <div>
           <form onsubmit={handle_submit}>
               <div class="grid grid-cols-4 gap-2 pr-4 items-center mb-1">
                   <div class="hidden">
                       <input
                           name="tier_id" 
                           value={tier_id.to_string()}
                       />
                   </div>
                   <div class="col-span-1">
                       <IdSelect
                           id="tier_agent"
                           options = {options}
                           options_id = {options_id}
                           selected = {agent_id}
                       >
                       </IdSelect>
                   </div>
                   <div class="col-span-1">
                       <Input
                           value={level.to_string()}
                           id="tier_level"
                       />
                   </div>
                   <div class="col-span-1">
                       <Input
                           value={pos.to_string()}
                           id="tier_position"
                       />
                   </div>
                   if tier_id == 0 {
                       <button class="btn btn-square btn-outline btn-sm text-end">
                           <Icon icon_id={IconId::LucideCheck}/>   
                       </button>
                   } else {
                       <div>
                           <button 
                                class="btn btn-square btn-outline btn-sm text-end" >
                                <Icon icon_id={IconId::LucideCheck}/>   
                           </button>
                           <div
                               onclick={handle_del}
                               class="btn btn-square btn-outline btn-sm text-end ml-1" >
                               <Icon icon_id={IconId::LucideTrash}/>   
                           </div>
                       </div>
                   }
               </div>             
           </form>
           <Dialog
               d_ref = {dialog_ref}
               title={"Warning!"} 
               contents={format!("Are you sure to delete the tier?")}
               onconfirm={handle_confirm}
               >
           </Dialog>          
        </div>
    }
}

#[function_component]
pub fn Queues() -> Html {
    let (store,_) = use_store::<Store>();
    let queues: UseStateHandle<Vec<Queue>> = use_state(||vec![]);
        
    {
        let store = store.clone();
        let queues = queues.clone();
        use_effect_with((), move|_| {
            let queues = queues.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_queues: Vec<Queue> =
                    Queue::list(store.selected_domain).await;
                queues.set(fetched_queues);
            });
        });
    }

    html!{
        <div class="grow mr-2">
            {
                queues.iter().map(|q|{
                    html!{
                        <Tiers
                            queue_name={q.name.clone()}
                            queue_id={q.id}
                        />
                    }
                }).collect::<Vec<Html>>()
            }
        </div>      
    }
}

#[function_component]
pub fn Tiers(props: &TiersProps) -> Html {
    let (store, _) = use_store::<Store>();
    let queue_id = props.queue_id;
    let tiers: UseStateHandle<Vec<Tier>> = use_state(||vec![]);
    let hidden_ref: NodeRef = use_node_ref();
    let count = use_state(||0);

    {
        let store = store.clone();
        let count = count.clone();
        let tiers = tiers.clone();
        use_effect_with(count, move |_| {
            let tiers = tiers.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("/callcenter/tier?queue_id={}", queue_id);
                let fetched_tiers =
                    Service::get(&url, store.selected_domain)
                        .await
                        .unwrap();
                tiers.set(fetched_tiers);
            });
        });
    };

    let handle_update = {
        let count = count.clone();
        let hidden_ref = hidden_ref.clone();
        Callback::from(move |_: i32|{
            let hidden_ref = hidden_ref.clone();
            let e = hidden_ref.cast::<HtmlInputElement>().unwrap();
            e.focus().unwrap();
            
            let c = *count + 1;
            count.set(c);
        })
    };

    html!{
        <div>
            <div class="divider my-1"></div>
            <input class="sr-only" ref={hidden_ref}/>      
            <div class="w-full grid grid-cols-3 gap-1">
                <Label>{props.queue_name.clone()}</Label>
                <div class="col-span-2">
                    <div class="grid grid-cols-4 gap-2 pr-4 text-xs font-bold mb-2">
                        <label class="mb-1">{"Agent"}</label>
                        <label class="mb-1">{"Level"}</label>
                        <label class="mb-1">{"Position"}</label>
                        <div>{""}</div>
                    </div>
                    <div>
                    {
                        for tiers.clone().iter().map(|tier| {
                            html!{
                                <TierComponent
                                    on_update={handle_update.clone()}
                                    tier_id={tier.id} 
                                    agent_id={tier.agent.id}
                                    queue_id={queue_id}
                                    agent_name={tier.agent.name.clone()} 
                                    level={tier.level} 
                                    position={tier.position} />
                            }
                        })                            
                    }
                    </div>
                    <div class="dropdown">
                        <div tabindex="0" role="button" class="btn m-1">{"Add"}</div>
                        <div
                            tabindex="0"
                            class="dropdown-content card card-sm bg-base-100 z-1 w-128 shadow-md">
                            <div class="grid grid-cols-4 gap-2 pr-4 text-xs font-bold mb-2">
                                <label class="mb-1">{"Agent"}</label>
                                <label class="mb-1">{"Level"}</label>
                                <label class="mb-1">{"Position"}</label>
                                <div>{""}</div>
                            </div>
                            <TierComponent 
                                on_update={handle_update}
                                tier_id={0} 
                                agent_id={0}
                                queue_id={queue_id}
                                agent_name={"".to_string()} 
                                level={1} 
                                position={1} />
                        </div>
                    </div>
                </div>               
            </div>
        </div>        
    }
}

pub fn tier_switch(route: TierRoute) -> Html {
    match route {
        TierRoute::Index => html!{<Queues />},
    }
}