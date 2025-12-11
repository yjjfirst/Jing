use yew::prelude::*;
use yew_router::prelude::*;
use yew_icons::{Icon, IconId};
use yewdux::{dispatch, use_store};
use util_macro::HashMapHelper;
use web_sys::FormData;

use crate::store::{Store};
use crate::models::Service;

use crate::components::header::Header;
use crate::components::label::Label;
use crate::components::input::Input;
use crate::components::action_buttons::ActionButtons;
use crate::components::user_select::UserSelect;


use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Agent {
    pub id: usize,
    pub user_id: usize,
    pub domain_id: usize,
    pub name: String,
    pub contact: String,
    pub params: HashMap<String, AgentParam>
}

impl Agent {
    pub fn new() -> Agent {
        Agent {
            id: 0,
            user_id: 0,
            domain_id: 0,
            name: "".to_string(),
            contact: "".to_string(),
            params: HashMap::new()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, HashMapHelper)]
pub struct AgentParam {
    pub id: i32,
    pub agent_id: i32,
    pub name: String,
    pub value: String,
}

impl AgentParam {
    pub fn _new() -> AgentParam {
        AgentParam { 
            id: 0, 
            agent_id: 0, 
            name: "".to_string(), 
            value: "".to_string(),
        }
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct AgentDetailsProps {
    #[prop_or(0)]
    pub id: usize
}

#[derive(Clone, Routable, PartialEq)]
pub enum AgentRoute {
    #[at("/callcenter/agent")]
    Index,
    #[at("/callcenter/agent/:id")]
    Get {id: usize},
}

#[derive(Clone, PartialEq, Properties)]
pub struct AgentListItemProps {
    pub id: usize,
    pub name: String,
    pub contact: String
}

#[function_component]
pub fn AgentListItem(props: &AgentListItemProps) -> Html {
    let nav = use_navigator().unwrap();
    let id = props.id;

    html! {
        <tr>
            <td>{props.name.clone()}</td>
            <td>{props.contact.clone()}</td>
            <td>
                <div class="flex justify-end">
                    <div 
                        onclick={
                            let nav = nav.clone();
                            Callback::from(move |_e: MouseEvent|{
                                nav.push(&AgentRoute::Get {id: id})
                            })
                        } 
                        class="btn btn-square btn-outline btn-sm mr-1">
                        <Icon icon_id={IconId::LucideEdit}/>   
                    </div>
                    <div
                        class="btn btn-square btn-outline btn-sm">
                        <Icon icon_id={IconId::LucideTrash}/>   
                    </div>
                </div>

            </td>
        </tr>
    }
}

#[function_component]
pub fn AgentList() -> Html {
    let loc = use_location().unwrap().clone();
    let agents: UseStateHandle<Vec<Agent>> = use_state(||vec![]);
    let nav = use_navigator().unwrap();
    let (store,_) = use_store::<Store>();

    {
        let loc = loc.clone();
        let agents = agents.clone();
        use_effect_with((), move |_| {
            let agents = agents.clone();
            let url = format!("{}", loc.path());
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_agents: Vec<Agent> =
                    Service::index(&url, store.selected_domain.clone())
                        .await
                        .unwrap();
                    agents.set(fetched_agents);
            });
        });
    }

    let handle_add: Callback<MouseEvent> = {
        let nav = nav.clone();
        Callback::from(move|_e: MouseEvent|{
            nav.push(&AgentRoute::Get {id: 0});        
        })
    };

    html!{
        <div class="grow mr-2">
            <table class="table table-zebra">
                <thead>
                    <tr>
                        <th>{"Name"}</th>
                        <th>{"Contact"}</th>
                    </tr>
                </thead>
                <tbody>
                {
                    agents.iter().map(|a|{
                        html! {
                            <AgentListItem
                                id={a.id}
                                name={a.name.clone()}
                                contact={a.contact.clone()}
                            >
                            </AgentListItem>
                        }
                    }).collect::<Vec<Html>>()
                }
                </tbody>
            </table>
            <div class="flex flex-row-reverse pr-4">
                <div onclick={handle_add} class="btn btn-square btn-outline btn-sm" >
                    <Icon icon_id={IconId::LucidePlus}/>   
                </div>
            </div>         
        </div> 
    }
}

#[function_component]
pub fn AgentDetails(props: &AgentDetailsProps) -> Html {
    let loc = use_location().unwrap();
    let nav = use_navigator().unwrap();
    let agent = use_state(||Agent::new());
    let (store, dispatch) = use_store::<Store>();

    {
        let store = store.clone();
        let agent = agent.clone();
        let loc = loc.clone();

        use_effect_with((), move |_| {
            let agent = agent.clone();
            let loc = loc.clone();
            let store = store.clone();

            wasm_bindgen_futures::spawn_local( async move {
                let fetched = 
                    Service::get(loc.path(), store.selected_domain)
                        .await
                        .unwrap();
                agent.set(fetched);
            });
        })
    }

    let handle_cancel = {
        let nav = nav.clone();
        Callback::from(move|_| {
            nav.push(&AgentRoute::Index);
        })
    };

    html! {
        <div class="grow mt-1">
            <Header title= {format!("Agent")}></Header>
            <div class="divider my-1"></div>         
            <form class="w-full">
                <div class="grid grid-cols-3 gap-1">
                    <Label>{"Name"}</Label>
                    <Input
                        value={agent.name.clone()}
                        id="name"
                    />
                    <Label>{"Contact"}</Label>
                    <UserSelect id="user" value={agent.contact.clone()}> 
                    </UserSelect>

                </div>
            <ActionButtons oncancel={handle_cancel}/>
            </form>
        </div>
    }
}

pub fn agent_switch(route: AgentRoute) -> Html {
    match route {
        AgentRoute::Index => html!{<AgentList />},
        AgentRoute::Get {id} => html!{<AgentDetails id={id} />}
    }
}
