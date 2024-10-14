use std::collections::HashMap;
use web_sys::{EventTarget, FormData, SubmitEvent, HtmlFormElement, HtmlDialogElement};
use wasm_bindgen::JsCast;

use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yew_icons::{Icon, IconId};
use crate::store::{show_alert, Store};

use crate::components::header::Header;
use crate::components::input::Input;
use crate::components::action_buttons::ActionButtons;
use crate::services::user::UserContainer;
use crate::services::Service;
use crate::services::user::*;
use crate::components::dialog::Dialog;

#[derive(Clone, Routable, PartialEq)]
pub enum UserRoute {
    #[at("/user")]
    Index,
    #[at("/user/:id")]
    Get {id: usize},
}

#[derive(Clone, PartialEq, Properties)] 
pub struct UserProps {
    pub user: User,
    pub ondel: Callback<usize>
}

#[derive(Clone, PartialEq, Properties)] 
pub struct UserDetailProps {
    pub id: usize,
}

#[function_component]
pub fn UserListItem(props: &UserProps) -> Html {
    let nav = use_navigator().unwrap();
    let user_props = props.clone();
    let dialog_ref: NodeRef = use_node_ref();
    let dd_ref = dialog_ref.clone();
    let loc: Location = use_location().unwrap().clone();
    let (store,_) = use_store::<Store>();    
    let ondel = props.ondel.clone();
    let id = props.user.id;

    let onedit: Callback<MouseEvent> = Callback::from(move |_e|{
        nav.push(&UserRoute::Get {id: user_props.user.id});
    });

    let onconfirm: Callback<bool> = Callback::from(move|_e: bool|{
        let loc = loc.clone();
        let store = store.clone();
        let ondel = ondel.clone();        
        wasm_bindgen_futures::spawn_local(async move {
            let path = format!("{}/{}", loc.path(), id);
            Service::delete(&path, store.clone().selected_domain).await;
            ondel.emit(id);
        })
    });

    let ondel = Callback::from(move|_e|{
        let d = dd_ref.cast::<HtmlDialogElement>().unwrap();
        d.show_modal().unwrap();  
    });

    html!{    
    <tr>
        <th>{user_props.user.user_id.clone()}</th>
        <th class="flex justify-end">
           <div onclick={onedit} class="mr-1">
                <div class="btn btn-square btn-outline btn-sm">
                    <Icon icon_id={IconId::LucideEdit}/>   
                </div>
            </div>
            <div onclick ={ondel}>
                <div class="btn btn-square btn-outline btn-sm">
                    <Icon icon_id={IconId::LucideTrash}/>   
                </div>
            </div>
        </th>
        <Dialog
            d_ref = {dialog_ref}
            title={"Warning!"} 
            contents={format!("Are you sure to delete the user: {}?", user_props.user.user_id.clone())}
            {onconfirm}
            >
        </Dialog>         
    </tr>
    }
}

#[function_component]
pub fn UserList() -> Html {
    let loc = use_location().unwrap().clone();    
    let (store,_) = use_store::<Store>();
    let extensions: UseStateHandle<Vec<User>> = use_state(||vec![]);
    let exts = extensions.clone();
    let users = extensions.clone();
    let nav = use_navigator().unwrap();

    use_effect_with((), move |_| {
        let exts  = exts.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_extensions: Vec<User> = 
                Service::index(loc.path(), store.selected_domain.clone()).await;
            exts.set(fetched_extensions);
        });
    });
    
    let ondel = Callback::from(move| id:usize|{
        let users = users.clone();
        let filtered: Vec<User> = users
            .iter()
            .filter(|u|{id != u.id})
            .map(|u|{u.clone()})
            .collect();
        
        users.set(filtered);
    });

    let onadd: Callback<MouseEvent> = Callback::from(move|_e: MouseEvent| {
        nav.push(&UserRoute::Get {id: 0});
    });

    let extensions_list: Vec<Html> = extensions.iter().map(|e|{
        html! {
            <UserListItem ondel={ondel.clone()} user={User {
                id: e.id, domain_id: e.domain_id, user_id: e.user_id.clone()
            }}></UserListItem>
        }
    }).collect();


    html! {
        <div class="grow mr-2">
            <Header title="Application -> User"></Header>
            <div class="divider my-1"></div>
            <table class="table table-zebra">
                <thead>
                    <th>{"User ID"}</th>
                </thead>
                <tbody>
                    {extensions_list}
                </tbody>
            </table>
            <div class="flex flex-row-reverse pr-4">
                <div onclick={onadd} class="btn btn-square btn-outline btn-sm" >
                    <Icon icon_id={IconId::LucidePlus}/>   
                </div>
            </div>             
        </div>
    }
}

pub fn update_var(name: &str, map: &mut HashMap<String, Var>, form_data: &FormData) 
{
    let value = &form_data.get(name).as_string().unwrap();
    let old = map.get(name);
    let mut user_id: usize = 0;
    let mut id: usize = 0;
    
    match old {
        Some(h) => {user_id = h.user_id; id = h.id;}
        _ => {}
    };

    map.insert(name.to_string(), Var {
        id, user_id,
        name: name.to_string(),
        value: value.to_string()

    });
}
pub fn update_param(name: &str, map: &mut HashMap<String, Param>, form_data: &FormData) 
{
    let value = &form_data.get(name).as_string().unwrap();
    let old = map.get(name);
    let mut user_id: usize = 0;
    let mut id: usize = 0;
    
    match old {
        Some(h) => {user_id = h.user_id; id = h.id;}
        _ => {}
    };

    map.insert(name.to_string(), Param {
        id, user_id,
        name: name.to_string(),
        value: value.to_string()

    });
}

pub fn get_var(name: &str, map: &HashMap<String, Var>) -> String
{
    match map.get(name) {
        Some(v) => v.value.clone(),
        None => "".to_string()
    }
}

pub fn get_param(name: &str, map: &HashMap<String, Param>) -> String
{
    match map.get(name) {
        Some(v) => v.value.clone(),
        None => "".to_string()
    }
}

#[function_component]
pub fn UserDetail(_props: &UserDetailProps) -> Html {
    let(store, dispatch) = use_store::<Store>();
    let cloned_store = store.clone();
    let user: UseStateHandle<UserContainer> = use_state(||UserContainer {
        user: User::new(),
        params: HashMap::new(),
        vars: HashMap::new()
    });

    let u = user.clone();
    let loc = use_location().unwrap();
    let loc_1 = use_location().unwrap();

    let nav = use_navigator().unwrap();

    use_effect_with((), move |_| {
        let user = u.clone();
        let loc = loc_1.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_user = Service::get(loc.path(), store.selected_domain).await;
            user.set(fetched_user);
        });
    });

    let form_oncancel = {
        let nav = nav.clone();
        Callback::from(move|_| {
            nav.push(&UserRoute::Index);
        })

    };

    let form_onsubmit = {
        let user = user.clone();
        let store = cloned_store.clone();
        Callback::from (move|event: SubmitEvent| {
            let dispatch = dispatch.clone();
            let store = store.clone();
            let loc = loc.clone();
            let nav = nav.clone();
            let user = user.clone();

            let target: Option<EventTarget> = event.target();
            let form = target.unwrap().dyn_into::<HtmlFormElement>().unwrap();            
            let form_data = FormData::new_with_form(&form).unwrap();

            let new_user = User {
                id: user.user.id,
                domain_id: user.user.domain_id,
                user_id: form_data.get("user_id").as_string().unwrap()
            };

            let mut new_vars = user.vars.clone();
            let mut new_params = user.params.clone();

            if new_user.id != 0 {
                update_param("password", &mut new_params, &form_data);
                update_param("vm-password", &mut new_params, &form_data);
                update_var("effective_caller_id_name", &mut new_vars, &form_data);
                update_var("effective_caller_id_number", &mut new_vars, &form_data);
                update_var("outbound_caller_id_name", &mut new_vars, &form_data);
                update_var("outbound_caller_id_number", &mut new_vars, &form_data);
            }

            let c = UserContainer {
                user: new_user,
                vars: new_vars,
                params: new_params
            };

            wasm_bindgen_futures::spawn_local(async move {
                let store = store.clone();
                show_alert("Updating ringing user.".to_string(), dispatch);
                Service::update(loc.path(), store.selected_domain, c).await;
                nav.push(&UserRoute::Index);            
            });

            event.prevent_default();
        })
    };

    html! {
        <div class="grow mr-2">
        <Header title= {format!("User ID: {}", user.user.clone().user_id.clone())}></Header>
        <div class="divider my-1"></div> 
            <form class="w-full" onsubmit={form_onsubmit}>
                <Input
                    value={user.user.clone().user_id.clone()}
                    id="user_id"
                    hidden={user.user.clone().user_id.clone() != ""}
                />
                if user.user.id != 0 {
                    <Input
                        value={get_param("password", &user.params)}
                        id="password"
                    />
                    <Input
                        value={get_var("effective_caller_id_name", &user.vars)}
                        id="effective_caller_id_name"
                    />
                    <Input
                            value={get_var("effective_caller_id_number", &user.vars)}
                            id="effective_caller_id_number"
                        />
                    <Input
                        value={get_var("outbound_caller_id_name", &user.vars)}
                        id="outbound_caller_id_name"
                    />
                    <Input
                        value={get_var("outbound_caller_id_number", &user.vars)}
                        id="outbound_caller_id_number"
                    />
                    <Input
                        value={get_param("vm-password", &user.params)}
                        id="vm-password"
                    />
                }
                <ActionButtons oncancel={form_oncancel} />
            </form>
        </div>
    }
}
pub fn user_switch(route: UserRoute) -> Html{
    match route {
        UserRoute::Index => html!{<UserList />},
        UserRoute::Get { id } => html !{<UserDetail id={id}/>}
    }
}