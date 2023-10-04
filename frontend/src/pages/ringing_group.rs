use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::components::header::{Header};
use crate::components::button::{Button, ButtonType};
use crate::services::ringing_group::{RingingGroup};
use crate::store::{show_alert, Store};
use crate::components::input::Input;

#[derive(Clone, Routable, PartialEq)]
pub enum RingingGroupsRoute {
    #[at("/ringing-group")]
    Index,
    #[at("/ringing-group/:id")]
    Get {id: String},
}

#[derive(Clone, PartialEq, Properties)] 
pub struct RingingGroupDetailsProps {
    pub id: String
}

#[function_component]
pub fn RingingGroups() -> Html {
    let ringing_groups: UseStateHandle<Vec<RingingGroup>> = use_state(||vec![]);
    let groups = ringing_groups.clone();
    use_effect_with_deps(move |_| {
        let groups = groups.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_groups: Vec<RingingGroup> = RingingGroup::fetch_all().await;
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

    let onclick: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
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
  let(_, dispatch) = use_store::<Store>();
  let id = props.id.parse::<usize>().unwrap();
    let group: UseStateHandle<RingingGroup> = use_state(||RingingGroup::new_empty());
    let g = group.clone();
    
    use_effect_with_deps(move |_| {
        let g = g.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_group: RingingGroup = RingingGroup::fetch(id).await;
            g.set(fetched_group);
        });
    },());
    
    let name_input_ref = use_node_ref();
    let name_input_ref_cloned = name_input_ref.clone();

    let exten_input_ref = use_node_ref();
    let exten_input_ref_cloned = exten_input_ref.clone();

    let desc_input_ref = use_node_ref();
    let desc_input_ref_cloned = desc_input_ref.clone();

    let ringtime_input_ref = use_node_ref();
    let ringtime_input_ref_cloned = ringtime_input_ref.clone();

    let ringstrategy_input_ref = use_node_ref();
    let ringstragegy_input_ref_cloned = ringstrategy_input_ref.clone();

    let onclick: Callback<MouseEvent> = Callback::from(move|_:MouseEvent|{
        let dispatch = dispatch.clone();
        let name_input: HtmlInputElement = name_input_ref_cloned.cast::<HtmlInputElement>().unwrap();
        let exten_input: HtmlInputElement = exten_input_ref_cloned.cast::<HtmlInputElement>().unwrap();
        let desc_input: HtmlInputElement = desc_input_ref_cloned.cast::<HtmlInputElement>().unwrap();
        let ringtime_input: HtmlInputElement = ringtime_input_ref_cloned.cast::<HtmlInputElement>().unwrap();
        let ringstrategy_input: HtmlInputElement = ringstragegy_input_ref_cloned.cast::<HtmlInputElement>().unwrap();
        let group = RingingGroup::new(
            id,
            name_input.value(),
            exten_input.value(),
            Some(desc_input.value()),
            1,           
            ringtime_input.value().parse::<i32>().unwrap(),
            ringstrategy_input.value()
        );
        wasm_bindgen_futures::spawn_local(async move {
          let dispatch = dispatch.clone();
          show_alert("Updating ringing group.".to_string(), dispatch);
          RingingGroup::update(id, group).await;
        });
        
    });

    html! { 
        <div class="grow">
            <Header title= {format!("Ringing Group: {}", group.group_id.clone())}></Header>
            <form class="w-full max-w-screen-lg">
            <div class="flex flex-wrap -mx-3 mb-6">
              <div class="w-full md:w-1/2 px-3 mb-6 md:mb-0">
                <Input 
                  label="Ringing Group Name" 
                  name="name" 
                  value={group.name.clone()}
                  input_type="text"
                  id="name"
                  input_ref={name_input_ref}/>
              </div>
              <div class="w-full md:w-1/2 px-3">
                <Input 
                  label="Ringing Group Extension" 
                  name="extension" 
                  value={group.group_id.clone()}
                  input_type="text"
                  id="extension"
                  input_ref={exten_input_ref}/>
              </div>
            </div>
            <div class="flex flex-wrap -mx-3 mb-6">
              <div class="w-full px-3">
                <Input 
                  label="Description" 
                  name="description" 
                  value={group.description.clone()}
                  input_type="text"
                  id="description"
                  input_ref={desc_input_ref}/>
              </div>
            </div>
            <div class="flex flex-wrap -mx-3 mb-6">
              <div class="w-full md:w-1/2 px-3 mb-6 md:mb-0">
                <Input 
                  label="Ring Time" 
                  name="ring-time" 
                  value={group.ring_time.to_string()}
                  input_type="number"
                  id="ring-time"
                  input_ref={ringtime_input_ref}/>
              </div>
              <div class="w-full md:w-1/2 px-3">
              <Input 
                  label="Ringing Strategy" 
                  name="ring-strategy" 
                  value={group.ring_strategy.clone()}
                  input_type="text"
                  id="ring-strategy"
                  input_ref={ringstrategy_input_ref}/>
              </div>
            </div>
          </form>
          <div class="flex">
          <div {onclick}>
            <Button b_type={ButtonType::Check}>{"Submit"}</Button>
          </div>
          <div>
            <Button b_type={ButtonType::X}>{"Cancel"}</Button>
          </div>
          </div>
        </div>
    }
}

pub fn ringinggroups_switch(route: RingingGroupsRoute) -> Html {
    match route {
        RingingGroupsRoute::Index => html!{ <RingingGroups />},
        RingingGroupsRoute::Get { id } => html!{<RingingGroupDetail id={id}/>}
    }
}