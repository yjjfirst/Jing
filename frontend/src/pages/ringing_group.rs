use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;
use yewdux::prelude::*;
use gloo_console::log;

use crate::components::header::Header;
use crate::components::button::{Button, ButtonIcon, ButtonTheme};
use crate::services::ringing_group::RingingGroup;
use crate::services::Service;
use crate::store::{show_alert, Store};
use crate::components::input::Input;
use crate::components::select::Select;

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
    let loc = use_location().unwrap().clone();    
    let (store,_) = use_store::<Store>();
    let s = store.clone();
    let ringing_groups: UseStateHandle<Vec<RingingGroup>> = use_state(||vec![]);
    let groups = ringing_groups.clone();
    use_effect_with_deps(move |_| {
        let groups = groups.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_groups: Vec<RingingGroup> = 
                Service::index(loc.path(), store.selected_domain.clone()).await;
            groups.set(fetched_groups);
        });
    },s.selected_domain.clone());
    
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
                <Button icon={ButtonIcon::Edit} theme={ButtonTheme::Light}></Button>
            </div>
        </div>
    }
}

#[function_component]
pub fn RingingGroupDetail(props: &RingingGroupDetailsProps) -> Html {
    let(store, dispatch) = use_store::<Store>();
    let store_cloned = store.clone();
    let id = props.id.parse::<usize>().unwrap();
    let group: UseStateHandle<RingingGroup> = use_state(||RingingGroup::new_empty());
    let g = group.clone();
    let loc = use_location().unwrap();
    let location = loc.clone();    
    
    use_effect_with_deps(move |_| {
        let g = g.clone();
        let loc = location.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_group: RingingGroup = Service::get(loc.path(), store.selected_domain).await;
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
        let store_cloned = store_cloned.clone();
        let loc = loc.clone();
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
            Service::update(loc.path(), store_cloned.selected_domain, group).await;
        });
        
    });

    let options: Vec<String> = vec![
        String::from("simultaneous"), 
        String::from("sequential")
    ];

    html! { 
        <div class="grow">
            <Header title= {format!("Ringing Group: {}", group.group_id.clone())}></Header>
            <form class="w-full max-w-screen-lg">
            <div class="flex flex-wrap -mx-3 mb-1">
              <div class="w-full md:w-1/2 px-3 mb-6 md:mb-0">
                <Input 
                  label="Ringing Group Name" 
                  name="name" 
                  value={group.name.clone()}
                  input_type="text"
                  id="name"
                  input_ref={name_input_ref}/>
              </div>
              <div class="w-full md:w-1/2 px-2">
                <Input 
                  label="Ringing Group Extension" 
                  name="extension" 
                  value={group.group_id.clone()}
                  input_type="text"
                  id="extension"
                  input_ref={exten_input_ref}/>
              </div>
            </div>
            <div class="flex flex-wrap -mx-3 mb-1">
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
            <div class="flex flex-wrap -mx-3 mb-1">
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
                <Select
                    {options}
                    select = {group.ring_strategy.to_string()}
                    name="Ring Strategy"
                    id="ring-strategy"
                    label="Ring Strategy"
                    input_ref={ringstrategy_input_ref}
                    >
                </Select>
              </div>
            </div>
            <div class="flex justify-end">
            <div {onclick}>
                <Button icon={ButtonIcon::Check} theme={ButtonTheme::Light}>{"Submit"}</Button>
            </div>
            <div>
                <Button icon={ButtonIcon::X} theme={ButtonTheme::Light}>{"Cancel"}</Button>
            </div>
          </div>            
          </form>

        </div>
    }
}

pub fn ringinggroups_switch(route: RingingGroupsRoute) -> Html {
    match route {
        RingingGroupsRoute::Index => html!{ <RingingGroups />},
        RingingGroupsRoute::Get { id } => html!{<RingingGroupDetail id={id}/>}
    }
}