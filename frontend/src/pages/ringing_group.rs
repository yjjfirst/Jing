use web_sys::{EventTarget, FormData, SubmitEvent, HtmlFormElement, HtmlDialogElement};
use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yew_icons::{Icon, IconId};
use wasm_bindgen::JsCast;

use crate::components::header::Header;
use crate::services::ringing_group::{RingingGroup, RingingGroupDetail};
use crate::services::Service;
use crate::store::{show_alert, Store};
use crate::components::input::Input;
use crate::components::select::Select;
use crate::components::mselect::Mselect;
use crate::services::user::User;
use crate::components::dialog::Dialog;

#[derive(Clone, Routable, PartialEq)]
pub enum RingingGroupsRoute {
    #[at("/ringing-group")]
    Index,
    #[at("/ringing-group/:id")]
    Get {id: usize},
}

#[derive(Clone, PartialEq, Properties)] 
pub struct RingingGroupDetailsProps {
    #[prop_or(0)]
    pub id: usize,
}

#[derive(Clone, PartialEq, Properties)]
pub struct RingingGroupListItemProps {
    pub id: usize,
    pub group_id: String,
    pub name: String,
    pub ondel: Callback<usize>
}

#[function_component]
pub fn RingingGroupList() -> Html {
    let loc = use_location().unwrap().clone();    
    let (store,_) = use_store::<Store>();
    let ringing_groups: UseStateHandle<Vec<RingingGroupDetail>> = use_state(||vec![]);
    let groups = ringing_groups.clone();
    let groups_1 = ringing_groups.clone();

    let nav = use_navigator().unwrap();

    use_effect_with((), move |_| {
        let groups = groups.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_groups: Vec<RingingGroupDetail> = 
                Service::index(loc.path(), store.selected_domain.clone()).await;
            groups.set(fetched_groups);
        });
    });
    
    let ondel = Callback::from(move |id: usize|{
        let groups = groups_1.clone();
        let filtered: Vec<&RingingGroupDetail> = groups
            .iter()
            .filter(|g|id != g.id)
            .collect();

        let filtered: Vec<RingingGroupDetail> = filtered
            .iter()
            .map(|g|{(**g).clone()})
            .collect();

        groups.set(filtered);
    });

    let groups: Vec<Html> = ringing_groups.iter().map(|g| html! {
        <RingingGroupListItem 
            ondel={ondel.clone()} 
            id={g.id} 
            group_id={g.group_id.clone()}
            name={g.name.clone()}>
        </RingingGroupListItem>
    }).collect();

    let onadd: Callback<MouseEvent> = Callback::from(move|_e: MouseEvent| {
        nav.push(&RingingGroupsRoute::Get {id: 0});
    });

    html! {
        <div class="grow mr-2">
            <Header title="Application -> Ringing Group"></Header>
            <div class="divider my-1"></div>
            <table class="table table-zebra">
                <thead>
                    <th>{"Extension"}</th>
                    <th>{"Name"}</th>
                </thead>
                <tbody>
                    {groups}
                </tbody>
            </table>
            <div class="flex flex-row-reverse pr-4">
                <div onclick={onadd} class="btn btn-square btn-outline btn-sm">
                    <Icon icon_id={IconId::LucidePlus}/>   
                </div>
            </div>            
        </div>
    }

}

#[function_component]
pub fn RingingGroupListItem(props: &RingingGroupListItemProps) -> Html {
    let props = props.clone();
    let id = props.id;
    let nav = use_navigator().unwrap();
    let loc: Location = use_location().unwrap().clone();
    let (store,_) = use_store::<Store>();
    let ondel = props.ondel.clone();

    let dialog_ref: NodeRef = use_node_ref();
    let dd_ref = dialog_ref.clone();
    
    let onconfirm: Callback<bool> = Callback::from(move |_e: bool|{
        let loc = loc.clone();
        let store = store.clone();
        let ondel = ondel.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let path = format!("{}/{}", loc.path(), id);
            Service::delete(&path, store.clone().selected_domain).await;
            ondel.emit(id);
        })
    });

    let onedit: Callback<MouseEvent> = Callback::from(move|_e: MouseEvent| {
        let nav = nav.clone();
        nav.push(&RingingGroupsRoute::Get {id});
    });

    let ondel: Callback<MouseEvent> = Callback::from(move|_e: MouseEvent| {
        let d = dd_ref.cast::<HtmlDialogElement>().unwrap();
        d.show_modal().unwrap();        
    });

    html! {
        <tr>
            <th>{props.group_id}</th>
            <th>{props.name}</th>
            <th class="flex justify-end">
                <div onclick={onedit} class="mr-1">
                    <div class="btn btn-square btn-outline btn-sm">
                        <Icon icon_id={IconId::LucideEdit}/>   
                    </div>
                </div>
                <div onclick={ondel}>
                    <div class="btn btn-square btn-outline btn-sm">
                        <Icon icon_id={IconId::LucideTrash}/>   
                    </div>
                </div>
            </th>
            <Dialog
                d_ref = {dialog_ref}
                title={"Warning!"} 
                contents={"Are you sure to delete Ringing Group"}
                {onconfirm}            
                >
            </Dialog>         
        </tr>
    }
}

#[function_component]
pub fn RingingGroupDetailComponent(props: &RingingGroupDetailsProps) -> Html {
    let(store, dispatch) = use_store::<Store>();
    let store_cloned = store.clone();

    let id = props.id;
    let group: UseStateHandle<RingingGroup> = use_state(||RingingGroup::new_empty());
    let g = group.clone();

    let loc = use_location().unwrap();
    let location = loc.clone(); 
    let nav = use_navigator().unwrap();

    let extensions: UseStateHandle<Vec<String>> = use_state(||vec![]);
    let es = extensions.clone();
    use_effect_with((), move |_| {
        let g = g.clone();
        let es = es.clone();
        let loc = location.clone();
        wasm_bindgen_futures::spawn_local(async move {
            if id != 0 {
                let fetched_group: RingingGroup = Service::get(loc.path(), store.clone().selected_domain).await;
                g.set(fetched_group);
            }
            let fetched_extensions = User::list(store.selected_domain).await;
            es.set(fetched_extensions);
        });
    });

    let options: Vec<String> = vec![
        String::from("simultaneous"), 
        String::from("sequential")
    ];

    let form_oncancel = {
        let nav = nav.clone();
        Callback::from(move|_| {
            nav.push(&RingingGroupsRoute::Index);
        })
    };

    let form_onsubmit = {
        let dispatch: Dispatch<Store> = dispatch.clone();
        let store_cloned = store_cloned.clone();
        let loc = loc.clone();
        let nav = nav.clone();
                
        Callback::from( move| event: SubmitEvent|{
            let nav = nav.clone();
            let dispatch: Dispatch<Store> = dispatch.clone();
            let store_cloned = store_cloned.clone();
            let loc = loc.clone();
                                
            let target: Option<EventTarget> = event.target();
            let form = target.unwrap().dyn_into::<HtmlFormElement>().unwrap();            
            let form_data = FormData::new_with_form(&form).unwrap();
            let members = form_data.get_all("members");

            let group = RingingGroup::new(
                id,
                form_data.get("name").as_string().unwrap(),
                form_data.get("extension").as_string().unwrap(),
                form_data.get("description").as_string().unwrap(),
                1,           
                form_data.get("ring-time").as_string().unwrap().parse::<i32>().unwrap(),
                form_data.get("ring-strategy").as_string().unwrap(),
                members.iter().map(|m|{
                    m.as_string().unwrap()
                }).filter(|m|{
                    !m.eq("")
                }).collect::<Vec<String>>()
            );

            wasm_bindgen_futures::spawn_local(async move {
                let dispatch = dispatch.clone();
                show_alert("Updating ringing group.".to_string(), dispatch);
                Service::update(loc.path(), store_cloned.selected_domain, group).await;
                nav.push(&RingingGroupsRoute::Index);            
            });
                        
            event.prevent_default();
        })
    };

    html! { 
        <div class="grow mr-2">
            <Header title= {format!("Ringing Group: {}", group.0.group_id.clone())}></Header>
            <div class="divider my-1"></div> 
            <form class="w-full" onsubmit={form_onsubmit}>
              <div class="w-full px-3 mb-6 md:mb-0">
                <Input
                  label="Ringing Group Name" 
                  name="name" 
                  value={group.0.name.clone()}
                  input_type="text"
                  id="name"
                  label_width="w-80"
                  />
              </div>
              <div class="w-full px-3">
                <Input 
                  label="Ringing Group Extension" 
                  name="extension" 
                  value={group.0.group_id.clone()}
                  input_type="text"
                  id="extension"
                  label_width="w-80"
                  />
              </div>
              <div class="w-full px-3">
                <Input 
                  label="Description" 
                  name="description" 
                  value={group.0.description.clone()}
                  input_type="text"
                  id="description"
                  label_width="w-80"
                  />
              </div>
              <div class="w-full px-3 mb-6 md:mb-0">
                <Input 
                  label="Ring Time" 
                  name="ring-time" 
                  value={group.0.ring_time.to_string()}
                  input_type="number"
                  id="ring-time"
                  label_width="w-80"
                  />
              </div>
              <div class="w-full px-3">
                <Select
                    {options}
                    select = {group.0.ring_strategy.to_string()}
                    name="ring-strategy"
                    id="ring-strategy"
                    label="Ring Strategy"
                    label_width="w-80"
                    >
                </Select>
              </div>
              <div class="w-full ">
            <Mselect 
                label_width_class="w-80"
                exists = {group.1.clone()}
                all = {extensions.iter().map(|e|e.to_string()).collect::<Vec<String>>()}
                >
            </Mselect>
            </div>
            <div class="flex justify-end mt-4">
            <div>
                <button class="btn btn-success btn-sm mr-4">
                    <Icon icon_id={IconId::LucideCheck}/>
                    {"Apply"}
                </button>
            </div>
            <div>
                <button class="btn btn-warning btn-sm"  onclick={form_oncancel}>
                    <Icon icon_id={IconId::LucideX}/>
                    {"Cancel"}
                </button>
            </div>
          </div>            
          </form>

        </div>
    }
}

pub fn ringinggroups_switch(route: RingingGroupsRoute) -> Html {
    match route {
        RingingGroupsRoute::Index => html!{ <RingingGroupList />},
        RingingGroupsRoute::Get { id } => html!{<RingingGroupDetailComponent id={id}/>}
    }
}