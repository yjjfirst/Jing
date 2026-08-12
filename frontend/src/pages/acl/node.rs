use yew::prelude::*;
use yew_router::prelude::*;
use yew::html::TargetCast;
use web_sys::HtmlInputElement;
use yew_icons::{Icon, IconData};
use yewdux::prelude::*;

use crate::models::Service;
use crate::store::{alert_info, alert_error, Store};

#[derive(Clone, PartialEq, Properties)]
pub struct NodeProps {
    pub node_type: String,
    pub cidr: String,
    pub acl_id: i32,
    pub node_id: i32,
    pub on_changed: Callback<()>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct NewNodeProps {
    pub acl_id: i32,
    pub on_changed: Callback<()>,
}

#[function_component]
pub fn Node(props: &NodeProps) -> Html {
    let (_, dispatch) = use_store::<Store>();
    let handle_delete = {
        let loc = use_location().unwrap();
        let on_changed = props.on_changed.clone();
        let node_id = props.node_id;
        let dispatch = dispatch.clone();
        Callback::from(move |_e: MouseEvent| {
            let loc = loc.clone();
            let on_changed = on_changed.clone();
            let dispatch = dispatch.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let path =format!("{}/node/{}", loc.path(), node_id);
                match Service::delete(&path, 0).await {
                    Ok(_) => {
                        alert_info("Node deleted successfully".to_string(), dispatch.clone());
                        on_changed.emit(());
                    },
                    Err(e) => {
                        alert_error(format!("Failed to delete node: {}", e), dispatch.clone());
                    }
                }
            })
        })
    };

    html!{
        <div class="grid grid-cols-3 w-full gap-1">
            <div>
                {props.cidr.clone()}
            </div>
            <div>
                {props.node_type.clone()}
            </div>
            <div onclick={handle_delete} class="btn btn-square btn-outline btn-sm">
                <Icon data={IconData::LUCIDE_TRASH}/>
            </div>
        </div>
    }
}

#[function_component]
pub fn NewNode(props: &NewNodeProps) -> Html {
    let loc = use_location().unwrap();
    let (store, dispatch) = use_store::<Store>();
    let domain_id = store.selected_domain_id;

    let cidr = use_state(||String::new());
    let node_type = use_state(||String::new());
    let acl_id = props.acl_id;

    let handle_cidr_change = {
        let cidr = cidr.clone();
        Callback::from(move |e: Event|{
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            cidr.set(value);
        })
    };

    let handle_type_change = {
        let node_type = node_type.clone();
        Callback::from(move|e: Event|{
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            node_type.set(value);
        })
    };

    let handle_add: Callback<MouseEvent> = {
        let cidr = cidr.clone();
        let node_type = node_type.clone();
        let loc = loc.clone();
        let on_changed = props.on_changed.clone();
        let dispatch = dispatch.clone();
        Callback::from(move |_| {
            let loc = loc.clone();
            let cidr = cidr.clone();
            let node_type = node_type.clone();
            let on_changed = on_changed.clone();
            let dispatch = dispatch.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let path =format!("{}/node/{}", loc.path(), 0);
                match Service::post(&path, domain_id, serde_json::json!({
                    "id": 0,
                    "acl_id": acl_id,
                    "cidr": cidr.to_string(),
                    "node_type": node_type.to_string(),

                })).await {
                    Ok(_) => {
                        alert_info("Node added successfully".to_string(), dispatch.clone());
                        on_changed.emit(());
                    },
                    Err(e) => {
                        alert_error(format!("Failed to add node: {}", e), dispatch.clone());
                    }
                }
            })
        })
    };

    html! {
        <div class="grid grid-cols-6 w-full gap-1">
            <input onchange={handle_cidr_change} class="pbx-input"/>
            <input onchange={handle_type_change} class="pbx-input"/>
            <div onclick={handle_add} class="btn btn-square btn-outline btn-sm">
                <Icon data={IconData::LUCIDE_PLUS}/>
            </div>
        </div>
    }
}
