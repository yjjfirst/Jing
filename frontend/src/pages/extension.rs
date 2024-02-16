use web_sys::{EventTarget, FormData, SubmitEvent, HtmlFormElement};
use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yew_icons::{Icon, IconId};

use crate::components::header::Header;
use crate::services::Service;
use crate::store::{show_alert, Store};
use crate::services::extension::Extension;

#[derive(Clone, Routable, PartialEq)]
pub enum ExtensionsRoute {
    #[at("/extension")]
    Index,
    #[at("/extension/:id")]
    Get {id: String},
}

#[derive(Clone, PartialEq, Properties)] 
pub struct ExtensionProps {
    pub id: usize,
    pub domain_id: i32,
    pub user_id: String    
}

#[derive(Clone, PartialEq, Properties)] 
pub struct ExtensionDetailProps {
    pub id: String,
}

#[function_component]
pub fn ExtensionsListItem(props: &Extension) -> Html {
    let nav = use_navigator().unwrap();
    let e = props.clone();
    let onedit: Callback<MouseEvent> = Callback::from(move |_e|{
        nav.push(&ExtensionsRoute::Get {id: e.id.clone().to_string()});
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
pub fn ExtensionsList() -> Html {
    let loc = use_location().unwrap().clone();    
    let (store,_) = use_store::<Store>();
    let extensions: UseStateHandle<Vec<Extension>> = use_state(||vec![]);
    let exts = extensions.clone();

    use_effect_with((), move |_| {
        let exts  = exts.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_extensions: Vec<Extension> = 
                Service::index(loc.path(), store.selected_domain.clone()).await;
            exts.set(fetched_extensions);
        });
    });
    
    let extensions_list: Vec<Html> = extensions.iter().map(|e|{
        html! {
            <ExtensionsListItem ..e.clone()></ExtensionsListItem>
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
pub fn ExtensionDetail(props: &ExtensionDetailProps) -> Html {
    let id = props.id.clone();
    html! {
        <div>{id}</div>
    }
}
pub fn extensions_switch(route: ExtensionsRoute) -> Html{
    match route {
        ExtensionsRoute::Index => html!{<ExtensionsList />},
        ExtensionsRoute::Get { id } => html !{<ExtensionDetail id={id}/>}
    }
}