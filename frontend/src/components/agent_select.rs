use yew::prelude::*;
use yewdux::prelude::*;
use crate::pages::callcenter::agents::model::Agent;
use crate::store::Store;

use crate::components::select_id::IdSelect;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
    pub selected_agent_id: usize
}

#[function_component]
pub fn AgentSelect(props: &Props) -> Html {
    let(store, _) = use_store::<Store>();
    let agents: UseStateHandle<Vec<Agent>> = use_state(||vec![]);
    let selected = use_state(||props.selected_agent_id);

    {
        let agents = agents.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_agents = Agent::list(store.selected_domain).await;
                agents.set(fetched_agents)
            });
        });
    }    
    
    let mut options = agents
        .iter()
        .map(|a|a.name.clone())
        .collect::<Vec<String>>();

    options.push("".to_string());

    let mut options_id = agents
        .iter()
        .map(|a|a.id)
        .collect::<Vec<usize>>();

    options_id.push(0);

    html! {
        <div>
           <IdSelect
               id={props.id.clone()}
               options = {options}
               options_id = {options_id}
               selected = {*selected}
           >
           </IdSelect>
        </div>
    }
}