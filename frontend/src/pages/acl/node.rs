use yew::prelude::*;

use super::model::AclNode;

#[derive(Clone, PartialEq, Properties)]
pub struct NodeProps {
    pub node_type: String,
    pub cidr: String,
}

#[function_component]
pub fn Node(props: &NodeProps) -> Html {
    let nodes: UseStateHandle<Vec<AclNode>> = use_state(||vec![]);
    wasm_bindgen_futures::spawn_local(async move {

    });

    html!{
        <div>{format!("node type: {}, cidr {}", props.node_type.clone(), props.cidr.clone())}</div>
    }
}
