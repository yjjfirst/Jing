use yew::prelude::*;
use yew_icons::{Icon, IconId};
use super::label::Label;
use super::input::Input;

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
            <div>
                <Input
                    id="ttt"
                    name="ttt"
                    value="ttt"
                    input_type="text"
                >
                </Input>
                <Input
                    id="ttt"
                    name="ttt"
                    value="ttt"
                    input_type="text"
                >
                </Input>
                <div class="btn btn-link btn-sm mr-4">
                    <Icon icon_id={IconId::LucidePlus}/>
                    {"Add"}
                </div>
            </div>
        </div>
    }
}