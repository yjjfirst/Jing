use yew::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Properties)]
pub struct RingingGroup {
    id: usize,
    name: String,
    group_id: String,
    description: Option<String>
}

#[function_component]
pub fn RingingGroups() -> Html {
    let ringing_groups: UseStateHandle<Vec<RingingGroup>> = use_state(||vec![]);
    {
        let ringing_groups = ringing_groups.clone();
        use_effect_with_deps(move |_| {
            let ringing_groups = ringing_groups.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_ringing_groups: Vec<RingingGroup> 
                    = Request::get("http://teleman.me:9090/api/ringing-groups")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                ringing_groups.set(fetched_ringing_groups);        
            });
        },());
    }
    
    ringing_groups.iter().map(|g| html! {
        <RingingGroupComponent ..g.clone()>
        </RingingGroupComponent>
    }).collect()

}

#[function_component]
pub fn RingingGroupComponent(props: &RingingGroup) -> Html {
    let props = props.clone();
    return html! {
        <div class="flex w-full hover:bg-zinc-200 duration-200 transition-colors border-b p-2">
            <div class="w-1/5">{props.group_id}</div>
            <div class="grow">{props.name}</div>
        </div>
    }
}