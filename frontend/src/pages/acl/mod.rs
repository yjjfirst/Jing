use std::collections::HashMap;
use web_sys::HtmlDialogElement;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;
use yew_icons::{Icon, IconId};
use yewdux::prelude::*;

use crate::components::header::Header;
use crate::components::dialog::Dialog;
use crate::models::Service;
use crate::store::Store;

#[derive(Clone, Routable, PartialEq)]
pub enum AclRoute {
    #[at("/acl")]
    Index,
    #[at("/acl/:id")]
    Get {id: usize},
}

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AclList {
    pub id: i32,
    pub acl_name: String,
    pub acl_default: String,
}

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AclNode {
    pub id: i32,
    pub list_id: Option<i32>,
    pub node_type: String,
    pub cidr: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct AclProps {
    pub acl: AclList,
    pub ondel: Callback<usize>
}

#[function_component]
pub fn AclListItem(props: &AclProps) -> Html {
    let acl = props.acl.clone();
    let nav = use_navigator().unwrap();
    let loc: Location = use_location().unwrap().clone();
    let (store,_) = use_store::<Store>();
    let ondel = props.ondel.clone();

    let dialog_ref: NodeRef = use_node_ref();
    let dd_ref = dialog_ref.clone();

    // clone small pieces to avoid moving the whole `acl` into multiple closures
    let acl_id_for_edit = acl.id as usize;
    let acl_id_for_confirm = acl.id as i32;
    let acl_name_for_display = acl.acl_name.clone();

    let onedit: Callback<MouseEvent> = Callback::from(move |_e|{
        nav.push(&AclRoute::Get {id: acl_id_for_edit});
    });

    let onconfirm: Callback<bool> = {
        let loc = loc.clone();
        let ondel = ondel.clone();
        let store_id = store.selected_domain_id;
        Callback::from(move |_e: bool|{
            let loc = loc.clone();
            let ondel = ondel.clone();
            let acl_id = acl_id_for_confirm;
            wasm_bindgen_futures::spawn_local(async move {
                let path = format!("{}/{}", loc.path(), acl_id);
                let _ = Service::delete(&path, store_id).await;
                ondel.emit(acl_id as usize);
            })
        })
    };

    let ondel_click: Callback<MouseEvent> = Callback::from(move |_e| {
        let d = dd_ref.cast::<HtmlDialogElement>().unwrap();
        d.show_modal().unwrap();
    });

    html! {
        <tr>
            <th>{acl_name_for_display.clone()}</th>
            <th class="flex justify-end">
                <div class="mr-1">
                    <div onclick={onedit} class="btn btn-square btn-outline btn-sm">
                        <Icon icon_id={IconId::LucideEdit}/>
                    </div>
                </div>
                <div>
                    <div onclick={ondel_click} class="btn btn-square btn-outline btn-sm">
                        <Icon icon_id={IconId::LucideTrash}/>
                    </div>
                </div>
            </th>
            <Dialog
                d_ref = {dialog_ref}
                title={"Warning!"}
                contents={format!("Are you sure to delete ACL: {}?", acl_name_for_display.clone())}
                {onconfirm}
                >
            </Dialog>
        </tr>
    }
}

#[function_component]
pub fn AclListPage() -> Html {
    let loc = use_location().unwrap().clone();
    let nav = use_navigator().unwrap();

    let (store,_) = use_store::<Store>();
    let acls: UseStateHandle<Vec<AclList>> = use_state(||vec![]);
    let acls_1 = acls.clone();
    let acls_2 = acls.clone();

    use_effect_with((), move|_|{
        let acls = acls_1.clone();
        let loc = loc.clone();
        let store = store.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched: Vec<AclList> = Service::index(loc.path(), store.selected_domain_id.clone()).await.unwrap_or_default();
            acls.set(fetched);
        });
    });

    let ondel: Callback<usize> = Callback::from(move|id: usize|{
        let acls = acls_2.clone();
        let filtered: Vec<AclList> = acls
            .iter()
            .filter(|g| {g.id as usize !=id})
            .map(|g|{g.clone()})
            .collect();

        acls.set(filtered);
    });

    let onadd: Callback<MouseEvent> = Callback::from(move|_e: MouseEvent|{
        nav.push(&AclRoute::Get {id: 0});
    });

    let acls_list: Vec<Html> = acls.iter().map(|g|{
        html! {
            <AclListItem acl={AclList {..g.clone()}} ondel={ondel.clone()}></AclListItem>
        }
    }).collect();

    html! {
        <div class="grow mr-2">
            <Header title="System -> ACL"></Header>
            <div class="divider my-1"></div>
            <table class="table table-zebra">
                <thead>
                    <tr>
                        <th>{"Name"}</th>
                    </tr>
                </thead>
                <tbody>
                {acls_list}
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

#[derive(Clone, PartialEq, Properties)]
pub struct AclDetailProps {
    pub id: usize,
}

#[function_component]
pub fn AclDetails(props: &AclDetailProps) -> Html {
    let id = props.id;
    let loc = use_location().unwrap();
    let (store,_) = use_store::<Store>();

    let acl = use_state(|| AclList { id:0, acl_name: "".to_string(), acl_default: "".to_string()});
    let nodes = use_state(|| vec![] as Vec<AclNode>);
    let acl_cloned = acl.clone();
    let nodes_cloned = nodes.clone();
    let loc_1 = loc.clone();
    use_effect_with((), move |_|{
        let acl = acl_cloned.clone();
        let nodes = nodes_cloned.clone();
        let loc = loc_1.clone();
        let domain = store.selected_domain_id;
        wasm_bindgen_futures::spawn_local(async move {
            let path = format!("{}/{}", loc.path(), id);
            let fetched: serde_json::Value = Service::get(&path, domain).await.unwrap_or(serde_json::json!(null));
            if let Some(list) = fetched.get("list").and_then(|v| v.as_object()) {
                let list: AclList = serde_json::from_value(serde_json::Value::Object(list.clone())).unwrap_or(AclList {id:0, acl_name: "".to_string(), acl_default: "".to_string()});
                acl.set(list);
            }
            if let Some(n) = fetched.get("nodes") {
                let nodes_vec: Vec<AclNode> = serde_json::from_value(n.clone()).unwrap_or_default();
                nodes.set(nodes_vec);
            }
        });
    });

    let nodes_html: Html = nodes.iter().map(|n|{
        html!{<tr><td>{n.cidr.clone()}</td><td>{n.node_type.clone()}</td></tr>}
    }).collect();

    html!{
        <div class="grow mr-2">
            <Header title={format!("System -> ACL -> {}", acl.acl_name.clone())}></Header>
            <div class="divider my-1"></div>
            <table class="table table-zebra">
                <thead>
                    <tr>
                        <th>{"CIDR"}</th>
                        <th>{"Type"}</th>
                    </tr>
                </thead>
                <tbody>
                    {nodes_html}
                </tbody>
            </table>
        </div>
    }
}

pub fn acl_switch(route: AclRoute) -> Html {
    match route {
        AclRoute::Index => html!{<AclListPage/>},
        AclRoute::Get { id } => html !{<AclDetails id={id}/>}
    }
}
