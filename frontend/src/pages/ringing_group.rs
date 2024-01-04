use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yew_icons::{Icon, IconId};

use crate::components::header::Header;
use crate::services::ringing_group::{RingingGroup, RingingGroupDetail};
use crate::services::Service;
use crate::store::{show_alert, Store};
use crate::components::input::Input;
use crate::components::select::Select;
use crate::components::mselect::Mselect;

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
    let ringing_groups: UseStateHandle<Vec<RingingGroupDetail>> = use_state(||vec![]);
    let groups = ringing_groups.clone();
    use_effect_with(s.selected_domain.clone(), move |_| {
        let groups = groups.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_groups: Vec<RingingGroupDetail> = 
                Service::index(loc.path(), store.selected_domain.clone()).await;
            groups.set(fetched_groups);
        });
    });
    
    let groups: Vec<Html> = ringing_groups.iter().map(|g| html! {
        <div>
            <RingingGroupListItem ..g.clone()>
            </RingingGroupListItem>
            <div class="divider my-1"></div>
        </div>
    }).collect();

    html! {
        <div class="grow mr-2">
            <Header title="Application -> Ringing Group"></Header>
            <div class="divider my-1"></div>
            {groups}
        </div>
    }

}

#[function_component]
pub fn RingingGroupListItem(props: &RingingGroupDetail) -> Html {
    let props = props.clone();
    let id = props.id;
    let nav = use_navigator().unwrap();

    let onclick: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        nav.push(&RingingGroupsRoute::Get {id: id.to_string()});
    });

    return html! {
        <div class="flex w-full items-center">
            <div class="w-1/5">{props.group_id}</div>
            <div class="grow">{props.name}</div>
            <div {onclick}>
                <div class="btn btn-square btn-outline btn-sm">
                    <Icon icon_id={IconId::LucideEdit}/>   
                </div>
            </div>
        </div>
    }
}

#[function_component]
pub fn RingingGroupDetailComponent(props: &RingingGroupDetailsProps) -> Html {
    let(store, dispatch) = use_store::<Store>();
    let store_cloned = store.clone();
    let id = props.id.parse::<usize>().unwrap();
    let group: UseStateHandle<RingingGroup> = use_state(||RingingGroup::new_empty());
    let g = group.clone();
    let loc = use_location().unwrap();
    let location = loc.clone();    
    
    use_effect_with((), move |_| {
        let g = g.clone();
        let loc = location.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_group: RingingGroup = Service::get(loc.path(), store.selected_domain).await;
            g.set(fetched_group);
        });
    });
    
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
            desc_input.value(),
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
        <div class="grow mr-2">
            <Header title= {format!("Ringing Group: {}", group.0.group_id.clone())}></Header>
            <div class="divider my-1"></div> 
            <form class="w-full max-w-screen-lg">
              <div class="w-full md:w-2/3 px-3 mb-6 md:mb-0">
                <Input
                  label="Ringing Group Name" 
                  name="name" 
                  value={group.0.name.clone()}
                  input_type="text"
                  id="name"
                  label_class="w-80"
                  input_ref={name_input_ref}/>
              </div>
              <div class="w-full md:w-2/3 px-3">
                <Input 
                  label="Ringing Group Extension" 
                  name="extension" 
                  value={group.0.group_id.clone()}
                  input_type="text"
                  id="extension"
                  label_class="w-80"
                  input_ref={exten_input_ref}/>
              </div>
              <div class="w-full md:w-2/3 px-3">
                <Input 
                  label="Description" 
                  name="description" 
                  value={group.0.description.clone()}
                  input_type="text"
                  id="description"
                  label_class="w-80"
                  input_ref={desc_input_ref}/>
              </div>
              <div class="w-full md:w-2/3 px-3 mb-6 md:mb-0">
                <Input 
                  label="Ring Time" 
                  name="ring-time" 
                  value={group.0.ring_time.to_string()}
                  input_type="number"
                  id="ring-time"
                  label_class="w-80"
                  input_ref={ringtime_input_ref}/>
              </div>
              <div class="w-full md:w-2/3 px-3">
                <Select
                    {options}
                    select = {group.0.ring_strategy.to_string()}
                    name="Ring Strategy"
                    id="ring-strategy"
                    label="Ring Strategy"
                    label_class="w-80"
                    input_ref={ringstrategy_input_ref}
                    >
                </Select>
              </div>
              <div class="w-full md:w-2/3">
            <Mselect 
                label_class="w-80"
                >
            </Mselect>
            </div>
            <div class="flex justify-end mt-4">
            <div {onclick}>
                <div class="btn btn-success btn-sm mr-4">
                    <Icon icon_id={IconId::LucideCheck}/>
                    {"Apply"}
                </div>
            </div>
            <div>
                <div class="btn btn-warning btn-sm">
                    <Icon icon_id={IconId::LucideX}/>
                    {"Cancel"}
                </div>
            </div>
          </div>            
          </form>

        </div>
    }
}

pub fn ringinggroups_switch(route: RingingGroupsRoute) -> Html {
    match route {
        RingingGroupsRoute::Index => html!{ <RingingGroups />},
        RingingGroupsRoute::Get { id } => html!{<RingingGroupDetailComponent id={id}/>}
    }
}