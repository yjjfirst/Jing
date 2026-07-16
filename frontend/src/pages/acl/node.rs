use yew::prelude::*;
use yew_router::prelude::*;
use yew::html::TargetCast;
use web_sys::HtmlInputElement;
use yew_icons::{Icon, IconId};

#[derive(Clone, PartialEq, Properties)]
pub struct NodeProps {
    pub node_type: String,
    pub cidr: String,
    pub acl_id: i32,
    pub node_id: i32,
}

#[function_component]
pub fn Node(props: &NodeProps) -> Html {
    html!{
        <div class="grid grid-cols-3 w-full gap-1">
            <div>
                {props.cidr.clone()}
            </div>
            <div>
                {props.node_type.clone()}
            </div>
            <div class="btn btn-square btn-outline btn-sm">
                <Icon icon_id={IconId::LucideTrash}/>
            </div>
        </div>
    }
}

#[function_component]
pub fn NewNode() -> Html {
    let loc = use_location().unwrap();
    let nav = use_navigator().unwrap();

    let cidr = use_state(||String::new());
    let node_type = use_state(||String::new());

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
        Callback::from(move |e| {
            let loc = loc.clone();
            let cidr = cidr.clone();
            let node_type = node_type.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let path =format!("{}/{}", loc.path(), 1);
            })
        })
    };

    html! {
        <div class="grid grid-cols-6 w-full gap-1">
            <input onchange={handle_cidr_change} class="pbx-input"/>
            <input onchange={handle_type_change} class="pbx-input"/>
            <div onclick={handle_add} class="btn btn-square btn-outline btn-sm">
                <Icon icon_id={IconId::LucidePlus}/>
            </div>
        </div>
    }
}
