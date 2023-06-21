use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;
use crate::header::{Header};
use crate::button::{Button, ButtonType};
use crate::app::{Env};

#[derive(Clone, Routable, PartialEq)]
pub enum RingingGroupsRoute {
    #[at("/ringing-group")]
    Index,
    #[at("/ringing-group/:id")]
    Get {id: String},
}


#[derive(Clone, PartialEq, Deserialize, Properties)]
pub struct RingingGroup {
    pub id: usize,
    pub name: String,
    pub group_id: String,
    pub description: Option<String>,
}

#[derive(Clone, PartialEq, Properties)] 
pub struct RingingGroupDetailsProps {
    pub id: String
}

#[function_component]
pub fn RingingGroups() -> Html {
    let env = use_context::<Env>().expect("no context found");
    let endpoint = format!("{}/ringing-groups", env.base_url);
    let ringing_groups: UseStateHandle<Vec<RingingGroup>> = use_state(||vec![]);
    let groups = ringing_groups.clone();
    use_effect_with_deps(move |_| {
        let groups = groups.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_groups: Vec<RingingGroup> 
                = Request::get(&endpoint)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
            groups.set(fetched_groups);        
        });
    },());
    
    let groups: Vec<Html> = ringing_groups.iter().map(|g| html! {
        <RingingGroupListItem ..g.clone()>
        </RingingGroupListItem>
    }).collect();

    html! {
        <div class="grow">
            <Header title="Application -> Ringing Group"></Header>
            {groups}
        </div>
    }

}

#[function_component]
pub fn RingingGroupListItem(props: &RingingGroup) -> Html {
    let props = props.clone();
    let id = props.id;
    let nav = use_navigator().unwrap();

    let onclick = Callback::from(move |_e: MouseEvent| {
        nav.push(&RingingGroupsRoute::Get {id: id.to_string()});
    });

    return html! {
        <div class="flex w-full hover:bg-skin-hover border-b h-12 items-center">
            <div class="w-1/5">{props.group_id}</div>
            <div class="grow">{props.name}</div>
            <div {onclick}>
                <Button b_type={ButtonType::Edit}></Button>
            </div>
        </div>
    }
}

#[function_component]
pub fn RingingGroupDetail(props: &RingingGroupDetailsProps) -> Html {
    let env = use_context::<Env>().expect("no context found");
    let id = props.clone().id;
    let endpoint = format!("{}/ringing-groups/{}", env.base_url, id);

    let group: UseStateHandle<RingingGroup> = use_state(||RingingGroup {
        id: 0,
        name: "".to_string(),
        group_id: "".to_string(),
        description: None
    });
    let g = group.clone();
    use_effect_with_deps(move |_| {
        let g = g.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_group: RingingGroup
                = Request::get(&endpoint)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
            g.set(fetched_group);
        });
    },());
    
    html! {
        <div class="grow">
            <Header title= {format!("Ringing Group: {}", id)}></Header>
            <p class="flex">{ format!("Ringing Group details: {}", id) }</p>
        </div>
    }
}

pub fn ringinggroups_switch(route: RingingGroupsRoute) -> Html {
    match route {
        RingingGroupsRoute::Index => html!{ <RingingGroups />},
        RingingGroupsRoute::Get { id } => html!{<RingingGroupDetail id={id}/>}
    }
}