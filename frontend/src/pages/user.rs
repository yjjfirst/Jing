use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yew_icons::{Icon, IconId};

use crate::components::header::Header;
use crate::components::input::Input;
use crate::services::user::WebUser;
use crate::services::Service;
use crate::store::Store;
use crate::services::user::*;

#[derive(Clone, Routable, PartialEq)]
pub enum UserRoute {
    #[at("/user")]
    Index,
    #[at("/user/:id")]
    Get {id: String},
}

#[derive(Clone, PartialEq, Properties)] 
pub struct UserProps {
    pub id: usize,
    pub domain_id: i32,
    pub user_id: String    
}

#[derive(Clone, PartialEq, Properties)] 
pub struct UserDetailProps {
    pub id: String,
}

#[function_component]
pub fn UserListItem(props: &User) -> Html {
    let nav = use_navigator().unwrap();
    let e = props.clone();
    let onedit: Callback<MouseEvent> = Callback::from(move |_e|{
        nav.push(&UserRoute::Get {id: e.id.clone().to_string()});
    });

    html!{    
    <div>
        <div class="flex w-full items-center">
            <div class="grow">{e.user_id.clone()}</div>
            <div onclick={onedit} class="mr-1">
                <div class="btn btn-square btn-outline btn-sm">
                    <Icon icon_id={IconId::LucideEdit}/>   
                </div>
            </div>
            <div>
                <div class="btn btn-square btn-outline btn-sm">
                    <Icon icon_id={IconId::LucideTrash}/>   
                </div>
            </div>             
        </div>
        <div class="divider my-1"></div>
    </div>
    }
}

#[function_component]
pub fn UserList() -> Html {
    let loc = use_location().unwrap().clone();    
    let (store,_) = use_store::<Store>();
    let extensions: UseStateHandle<Vec<User>> = use_state(||vec![]);
    let exts = extensions.clone();

    use_effect_with((), move |_| {
        let exts  = exts.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_extensions: Vec<User> = 
                Service::index(loc.path(), store.selected_domain.clone()).await;
            exts.set(fetched_extensions);
        });
    });
    
    let extensions_list: Vec<Html> = extensions.iter().map(|e|{
        html! {
            <UserListItem ..e.clone()></UserListItem>
        }
    }).collect();

    html! {
        <div class="grow mr-2">
            <Header title="Application -> Extension"></Header>
            <div class="divider my-1"></div>
            {extensions_list}
            <div class="flex flex-row-reverse">
                <div class="btn btn-square btn-outline btn-sm">
                    <Icon icon_id={IconId::LucidePlus}/>   
                </div>
            </div>             
        </div>
    }
}

#[function_component]
pub fn UserDetail(props: &UserDetailProps) -> Html {
    let(store, dispatch) = use_store::<Store>();
    let user: UseStateHandle<WebUser> = use_state(||WebUser {
        user: User::new(),
        params: vec![],
        vars: vec![]
    });

    let id: i32= props.id.clone().parse().unwrap();

    use_effect_with((), move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let exten = User::get(store.selected_domain, id).await;
        });
    });

    html! {
        <div class="grow mr-2">
        <Header title= {format!("Extension: {}", id.clone())}></Header>
        <div class="divider my-1"></div> 
            <form class="w-full">
                <div class="w-full px-3 mb-6 md:mb-0">
                    <Input
                        label="Ringing Group Name" 
                        name="name" 
                        value={"asdfsad"}
                        input_type="text"
                        id="name"
                        label_width="w-80"
                    />
                </div>
                <div class="flex justify-end mt-4">
                    <div>
                        <button class="btn btn-success btn-sm mr-4">
                            <Icon icon_id={IconId::LucideCheck}/>
                            {"Apply"}
                        </button>
                    </div>
                    <div>
                        <button class="btn btn-warning btn-sm" >
                            <Icon icon_id={IconId::LucideX}/>
                            {"Cancel"}
                        </button>
                    </div>
                </div>
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