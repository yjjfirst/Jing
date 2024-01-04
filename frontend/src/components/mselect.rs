use yew::prelude::*;
use yew_icons::{Icon, IconId};
use super::label::Label;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label_class: Classes,
}

#[function_component]
pub fn Mselect(props: &Props) -> Html {
    let class: Classes = props.label_class.clone();

    html! {
        <div class="flex w-full px-3">
            <Label class={class}>
            <span class="label-text">
                {"Extensions"}
            </span>
            </Label>
            <div class="w-full flex mr-1 py-4 border border-neutral rounded-lg">
                <button class="btn btn-outline btn-sm mx-1">
                    {"1001"}
                    <Icon icon_id={IconId::LucideX}/>
                </button>
                <button class="btn btn-outline btn-sm mx-1">
                    {"1001"}
                    <Icon icon_id={IconId::LucideX}/>
                </button>
                <button class="btn btn-outline btn-sm mx-1">
                    {"1001"}
                    <Icon icon_id={IconId::LucideX}/>
                </button>
                <div>              
                <button class="btn btn-square btn-outline btn-sm">
                    <Icon icon_id={IconId::LucidePlus}/>
                </button>
                </div>
            </div>
        </div>
    }
}