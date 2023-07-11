use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;
use crate::components::header::{Header};
use crate::components::button::{Button, ButtonType};
use crate::services::{RingingGroup};

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
            ringtime_input.value().parse::<i32>().unwrap(),
            ringstrategy_input.value()
        );
        wasm_bindgen_futures::spawn_local(async move {RingingGroup::update(id, group).await;});
    });

    html! { 
        <div class="grow">
            <Header title= {format!("Ringing Group: {}", group.group_id.clone())}></Header>
            <form class="w-full max-w-screen-lg">
            <div class="flex flex-wrap -mx-3 mb-6">
              <div class="w-full md:w-1/2 px-3 mb-6 md:mb-0">
                <label 
                  class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2" 
                  for="name">
                  {"Ringing Group Name"}
                </label>
                <input 
                  class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:bg-white" 
                  id="name" 
                  type="text"
                  value={group.name.clone()}
                  ref={name_input_ref}/>
              </div>
              <div class="w-full md:w-1/2 px-3">
                <label class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2" 
                  for="extension">
                  {"Ringing Groue Extension"}
                </label>
                <input class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 leading-tight focus:outline-none focus:bg-white focus:border-gray-500" 
                  id="extension" 
                  type="text" 
                  value={group.group_id.clone()}
                  ref={exten_input_ref}/>
              </div>
            </div>
            <div class="flex flex-wrap -mx-3 mb-6">
              <div class="w-full px-3">
                <label class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2" 
                  for="description">
                  {"Description"}
                </label>
                <input class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:bg-white focus:border-gray-500" 
                  id="description" 
                  type="text" 
                  placeholder={group.description.clone()}
                  ref={desc_input_ref}/>
              </div>
            </div>
            <div class="flex flex-wrap -mx-3 mb-6">
              <div class="w-full md:w-1/2 px-3 mb-6 md:mb-0">
                <label 
                  class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2" 
                  for="ring-time">
                  {"Ring Time"}
                </label>
                <input 
                  class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:bg-white" 
                  id="ring-time" 
                  type="text" 
                  value={group.ring_time.to_string()}
                  ref={ringtime_input_ref}/>
              </div>
              <div class="w-full md:w-1/2 px-3">
                <label class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2" 
                  for="ring-strategy">
                  {"Ringing Strategy"}
                </label>
                <input class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 leading-tight focus:outline-none focus:bg-white focus:border-gray-500" 
                  id="ring-strategy" 
                  type="text" 
                  value={group.ring_strategy.clone()}
                  ref={ringstrategy_input_ref}/>
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