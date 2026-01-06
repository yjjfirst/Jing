use yew::prelude::*;
use yewdux::prelude::*;
use crate::pages::callcenter::agents::model::Agent;
use crate::store::Store;

use crate::components::select::Select;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
    pub value: String
}

#[function_component]
pub fn AgentSelect(props: &Props) -> Html {
    let(store, _) = use_store::<Store>();

    let agents: UseStateHandle<Vec<String>> = use_state(||vec![]);

    {
        let agents = agents.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_agents = Agent::list(store.selected_domain).await;
                agents.set(fetched_agents)
            });
        });
    }    
    
    html! {
        <Select
            id={props.id.clone()}
            options = {agents.iter().map(|e|e.to_string()).collect::<Vec<String>>()}
            selected = {props.value.clone()}
        >
        </Select>

    }
}