pub mod model;

use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yew_icons::{Icon, IconId};

use crate::models::Service;
use model::Tier;
use crate::store::Store;
use crate::pages::callcenter::agents::model::Agent;
use crate::pages::callcenter::queues::model::Queue;

use crate::components::input::Input;
use crate::components::label::Label;
use crate::components::action_buttons::{ActionButtons};
use crate::components::agent_select::AgentSelect;

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
    pub id: i32,
    pub name: String,
    pub level: i32,
    pub position: i32,
}

#[function_component]
pub fn TierComponent(props: &TierComponentProps) -> Html {
    let handle_del: Callback<MouseEvent> = {
        Callback::from(move |_e| {
        })
    };

    html!{
        <div class="grid grid-cols-4 gap-2 pr-4 items-center mb-1">
            <div class="col-span-1">
                <AgentSelect
                    value={props.name.clone()}
                    id="tier_agent"
                />
            </div>
            <div class="col-span-1">
                <Input
                    value={props.level.to_string()}
                    id="tier_level"
                />
            </div>
            <div class="col-span-1">
                <Input
                    value={props.position.to_string()}
                    id="tier_position"
                />
            </div>            
            <div
                onclick={handle_del}
                class="btn btn-square btn-outline btn-sm ml-2">
                <Icon icon_id={IconId::LucideTrash}/>   
            </div>            
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

    {
        let tiers = tiers.clone();
        use_effect_with((), move |_| {
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

    let onadd =  {
        let tiers = tiers.clone();  
        Callback::from(move|_e: MouseEvent|{
            let tiers = tiers.clone();
            tiers.set({
                let mut v = (*tiers).clone();
                v.push(Tier {
                    id: 0,
                    queue_id,
                    level: 1,
                    position: 1,
                    agent: Agent::new()
                });
                v
            });
        })
    };
    html!{
        <div>
            <div class="divider my-1"></div>         
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
                        for tiers.iter().map(|tier| {
                            html!{
                                <TierComponent 
                                    id={tier.id} 
                                    name={tier.agent.name.clone()} 
                                    level={tier.level} 
                                    position={tier.position} />
                            }
                        })
                    }
                    </div>
                    <div class="flex pr-4 mt-2 items-center justify-between">
                        <div class="btn btn-square btn-outline btn-sm mt-4" >
                            <Icon onclick={onadd} icon_id={IconId::LucidePlus}/>   
                        </div>
                        <ActionButtons has_cancel={false}/>      
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