pub mod model;

use web_sys::{HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yew_icons::{Icon, IconData};
use serde_json::json;

use crate::models::Service;
use model::Tier;
use crate::store::{Store};
use crate::pages::callcenter::agents::model::Agent;
use crate::components::select_id::SelectId;

#[derive(Properties, PartialEq)]
pub struct TiersProps {
    pub queue_name: String,
    pub queue_id: i32,
}

#[derive(Properties, PartialEq)]
pub struct NewTierProps {
    pub queue_id: i32,
    pub on_changed: Callback<()>
}

#[derive(Properties, PartialEq)]
pub struct TierComponentProps {
    pub tier_id: i32,
    pub agent_id: usize,
    pub queue_id: i32,
    pub agent_name: String,
    pub level: i32,
    pub position: i32,
    pub on_changed: Callback<()>
}

#[function_component]
pub fn TierComponent(props: &TierComponentProps) -> Html {
    let tier_id = props.tier_id;
    let on_changed = props.on_changed.clone();
    let(store, _) = use_store::<Store>();
    let agents: UseStateHandle<Vec<Agent>> = use_state(||vec![]);

    {
        let store = store.clone();
        let agents = agents.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_agents = Agent::list(store.selected_domain_id).await;
                agents.set(fetched_agents)
            });
        });
    }

    let handle_del = {
        let store = store.clone();
        let on_changed = on_changed.clone();
        Callback::from(move |_e: MouseEvent| {
            let store = store.clone();
            let on_changed = on_changed.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let path = format!("/callcenter/tier/{}", tier_id);
                Service::delete(&path, store.clone().selected_domain_id)
                    .await
                    .unwrap();
                on_changed.emit(());
            });
        })
    };

    html!{
        <>
            <div>{props.agent_name.clone()}</div>
            <div>{props.level}</div>
            <div>{props.position}</div>
            <button class="btn btn-square btn-outline btn-sm"
                onclick={handle_del.clone()}
                type="button"
            >
                <Icon data={IconData::LUCIDE_TRASH}/>
            </button>
        </>
    }
}

#[component]
pub fn NewTier(props: &NewTierProps) -> Html {
    let(store, _) = use_store::<Store>();
    let agents: UseStateHandle<Vec<Agent>> = use_state(||vec![]);
    let level = use_state(||0);
    let queue_id = props.queue_id;
    let position = use_state(||0);
    let agent_id = use_state(||0);

    {
        let store = store.clone();
        let agents = agents.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_agents = Agent::list(store.selected_domain_id).await;
                agents.set(fetched_agents)
            });
        });
    }

    let handle_level_change = {
        let level = level.clone();
        Callback::from(move |e: InputEvent| {
            let level = level.clone();
            let input = e.target_unchecked_into::<HtmlInputElement>();
            if let Some(value) = input.value().parse::<i32>().ok() {
                level.set(value);
            }
        })
    };

    let handle_agent_change = {
        let agent_id = agent_id.clone();
        Callback::from(move |s: String| {
            let agent_id = agent_id.clone();
            if let Some(value) = s.parse::<usize>().ok() {
                agent_id.set(value);
            }
        })
    };

    let handle_position_change = {
        let position = position.clone();
        Callback::from(move |e: InputEvent| {
            let position = position.clone();
            let input = e.target_unchecked_into::<HtmlInputElement>();
            if let Some(value) = input.value().parse::<i32>().ok() {
                position.set(value);
            }
        })
    };

    let handle_add = {
        let store = store.clone();
        let agent_id = agent_id.clone();
        let level = level.clone();
        let position = position.clone();
        let on_changed = props.on_changed.clone();
        Callback::from(move |_| {
            let store = store.clone();
            let url = format!("/callcenter/tier/0");
            let level = level.clone();
            let position = position.clone();
            let on_changed = on_changed.clone();
            let agent_id = agent_id.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let mut agent = Agent::new();
                agent.id = *agent_id;
                Service::post(&url, store.selected_domain_id, json!({
                    "id": 0,
                    "queue_id": queue_id,
                    "level": *level,
                    "position": *position,
                    "agent": agent
                }))
                    .await
                    .unwrap();
                on_changed.emit(());
            });
        })
    };

    html! {
        <div class="grid grid-cols-4 gap-2 mt-2 items-center">
            <SelectId
                 name="tier_agent"
                 options = {
                     agents
                         .iter()
                         .map(|a|a.name.clone())
                         .collect::<Vec<String>>()
                 }
                 options_id = {
                     agents
                         .iter()
                         .map(|a|a.id)
                         .collect::<Vec<usize>>()
                 }
                 selected = {0}
                 onchange = {handle_agent_change}
            >
            </SelectId>
            <input class="input"
                placeholder="Level"
                name="tier_level"
                oninput={handle_level_change.clone()}
            />
            <input class="input"
                placeholder="Position"
                name="tier_position"
                oninput={handle_position_change.clone()}
            />
            <button
                class="btn btn-square btn-primary btn-xs text-end"
                type="button"
                onclick={handle_add}
            >
                <Icon data={IconData::LUCIDE_PLUS}/>
            </button>
        </div>
    }
}

#[function_component]
pub fn Tiers(props: &TiersProps) -> Html {
    let (store, _) = use_store::<Store>();
    let queue_id = props.queue_id;
    let tiers: UseStateHandle<Vec<Tier>> = use_state(||vec![]);
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
                    Service::get(&url, store.selected_domain_id)
                        .await
                        .unwrap();
                tiers.set(fetched_tiers);
            });
        });
    };

    let handle_changed = {
        let count = count.clone();
        Callback::from(move |_|{
            let c = *count + 1;
            count.set(c);
        })
    };

    html!{
        <div class="col-span-2">
            <div class="grid grid-cols-4 w-full gap-1">
            <div>{"Contact"}</div>
            <div>{"Level"}</div>
            <div>{"Position"}</div>
            <div>{""}</div>
            {
                for tiers.clone().iter().map(|tier| {
                    html!{
                        <TierComponent
                            on_changed={handle_changed.clone()}
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
            <NewTier queue_id={queue_id} on_changed={handle_changed.clone()}/>
        </div>
    }
}
