use yew::prelude::*;
use yew_router::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;
use crate::header::{Header};
use crate::button::{Button, ButtonType};

#[derive(Clone, Routable, PartialEq)]
pub enum RingingGroupsRoute {
    #[at("/ringing-group")]
    Index,
    #[at("/ringing-group/ttt")]
    Get,
}


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
    
    let groups: Vec<Html> = ringing_groups.iter().map(|g| html! {
        <RingingGroupComponent ..g.clone()>
        </RingingGroupComponent>
    }).collect();

    html! {
        <div class="grow">
            <Header title="Application->Ringing Group"></Header>
            {groups}
        </div>
    }

}

#[function_component]
pub fn RingingGroupComponent(props: &RingingGroup) -> Html {
    let props = props.clone();
    return html! {
        <div class="flex w-full hover:bg-skin-hover border-b h-12 items-center">
            <div class="w-1/5">{props.group_id}</div>
            <div class="grow">{props.name}</div>
            <Button b_type={ButtonType::Edit}></Button>
        </div>
    }
}

#[function_component]
pub fn RingingGroupDetail() -> Html {
    html! {
        <p class="flex">{"Ringing Group details"}</p>
    }
}

pub fn switch_ringinggroups(route: RingingGroupsRoute) -> Html {
    match route {
        RingingGroupsRoute::Index => html!{ <RingingGroups />},
        RingingGroupsRoute::Get  => html!{<RingingGroupDetail/>}
    }
}