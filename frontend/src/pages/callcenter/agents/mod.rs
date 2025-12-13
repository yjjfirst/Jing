pub mod agent;

use std::collections::HashMap;
use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{FormData, SubmitEvent, HtmlDialogElement};
use yew_router::prelude::*;
use yew_icons::{Icon, IconId};
use yewdux::prelude::*;

use crate::store::{alert_info, alert_error, Store};
use crate::models::Service;

use crate::components::header::Header;
use crate::components::label::Label;
use crate::components::input::Input;
use crate::components::action_buttons::ActionButtons;
use crate::components::user_select::UserSelect;
use crate::components::select::Select;
use crate::components::dialog::Dialog;

use agent::{Agent, AgentParam};

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
    pub contact: String,
    pub ondel: Callback<usize>
}

#[function_component]
pub fn AgentListItem(props: &AgentListItemProps) -> Html {
    let loc: Location = use_location().unwrap();
    let nav = use_navigator().unwrap();
    let dialog_ref: NodeRef = use_node_ref(); 
    let (store,_) = use_store::<Store>();
    let ondel = props.ondel.clone();
    
    let id = props.id;

    let onconfirm: Callback<bool> = Callback::from(move|_e: bool|{  
        let loc = loc.clone();
        let store = store.clone();
        let ondel = ondel.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let path = format!("{}/{}", loc.path(), id);
            Service::delete(&path, store.clone().selected_domain)
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
                        onclick={handle_del}
                        class="btn btn-square btn-outline btn-sm">
                        <Icon icon_id={IconId::LucideTrash}/>   
                    </div>
                </div>

            </td>
            <Dialog
                d_ref = {dialog_ref}
                title={"Warning!"} 
                contents={format!("Are you sure to delete the Queue: {}?", props.name.clone())}
                {onconfirm}
                >
            </Dialog>              
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

    let handle_del: Callback<usize>  = {
        let agents = agents.clone();
        Callback::from(move | id: usize| {
            let mut new_agents = (*agents).clone();
            new_agents.retain(|a| a.id != id);
            agents.set(new_agents);    
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
                                ondel={handle_del.clone()}
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
pub fn AgentDetails(_props: &AgentDetailsProps) -> Html {
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

    let handle_submit = {
        let loc = use_location().unwrap();
        let agent = agent.clone();
        let store = store.clone();
        let nav = nav.clone();
        let dispatch = dispatch.clone();
        let id = agent.id;

        Callback::from(move |event: SubmitEvent| {
            let loc = loc.clone();
            let target = event.target().unwrap();
            let form = target.dyn_into().unwrap();            
            let form_data = FormData::new_with_form(&form).unwrap();
            let dispatch = dispatch.clone();
            let nav = nav.clone();
            let store = store.clone();

            event.prevent_default();

            let params = agent.params.clone()
                .into_iter()
                .map(|p|{
                    let key = p.0;
                    let mut param = p.1;
                    gloo_console::log!("Key:", &key);
                    param.value = form_data.get(&key).as_string().unwrap();
                    (key, param)
                })
                .collect::<HashMap<String, AgentParam>>();
            
            let data = Agent {
                id: agent.id,
                user_id: agent.user_id,
                domain_id: agent.domain_id,
                name: form_data.get("name").as_string().unwrap(),
                contact: form_data.get("user").as_string().unwrap(),
                leg_timeout: form_data.get("leg-timeout").as_string().unwrap().parse::<i32>().unwrap(),
                params
            };

            wasm_bindgen_futures::spawn_local(async move {
                let dispatch = dispatch.clone();
                match Service::post(loc.path(), store.selected_domain, data).await {
                    Ok(_) => {
                        if id == 0 {
                            alert_info("Agent created successfully.".to_string(), dispatch);
                        } else {
                            alert_info("Agent updated successfully.".to_string(), dispatch);
                        }
                    },
                    Err(e) => {
                        if id == 0 {
                            alert_error(format!("Failed to create agent: {}", e), dispatch);
                        } else {
                            alert_error(format!("Failed to update agent: {}", e), dispatch);
                        }
                    }
                }
                nav.push(&AgentRoute::Index);
            });
        })
    };

    html! {
        <div class="grow mt-1">
            <Header title= {format!("Agent")}></Header>
            <div class="divider my-1"></div>         
            <form class="w-full" onsubmit={handle_submit}>
                <div class="grid grid-cols-3 gap-1">
                    <Label>{"Name"}</Label>
                    <Input
                        value={agent.name.clone()}
                        id="name"
                    />
                    <Label>{"Contact"}</Label>
                    <UserSelect id="user" value={agent.contact.clone()}> 
                    </UserSelect>
                    <Label>{"Leg Timeout"}</Label>
                    <Input 
                        value={agent.leg_timeout.to_string()}
                        id="leg-timeout"/>
                    <Label>{"Wrap Up Time"}</Label>       
                    <Input
                        value={AgentParam::get("wrap-up-time", &agent.params)}
                        id="wrap-up-time" />
                    <Label>{"Max no answer"}</Label>
                    <Input
                        value={AgentParam::get("max-no-answer", &agent.params)}
                        id="max-no-answer" />
                    <Label>{"Reject delay time"}</Label>
                    <Input
                        value={AgentParam::get("reject-delay-time", &agent.params)}
                        id="reject-delay-time" />
                    <Label>{"Busy delay time"}</Label>
                    <Input
                        value={AgentParam::get("busy-delay-time", &agent.params)}
                        id="busy-delay-time" />
                    <Label>{"Type"}</Label>
                    <Input
                        value={AgentParam::get("type", &agent.params)}
                        id="type" />
                    <Label>{"Status"}</Label>
                    <Select
                        id="status"
                        selected={AgentParam::get("status", &agent.params)}
                        options={vec![ 
                            "Logged Out".to_string(),
                            "Available".to_string(),
                            "Available (On Demand)".to_string(),
                            "On Break".to_string()]} >
                    </Select>
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
