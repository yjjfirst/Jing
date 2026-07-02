use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Clone, PartialEq, Properties)]
pub struct NodeProps {
    pub node_type: String,
    pub cidr: String,
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
    html! {
        <div class="grid grid-cols-6 w-full gap-1">
            <input class="pbx-input"/>
            <input class="pbx-input"/>
            <div class="btn btn-square btn-outline btn-sm">
                <Icon icon_id={IconId::LucidePlus}/>
            </div>
        </div>
    }
}
