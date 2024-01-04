use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LabelProps {
    pub class: Classes,
    pub children: Html,
}

#[function_component]
pub fn Label(props: &LabelProps) -> Html {
    let mut class = props.class.clone();
    class.push("label-text shrink-0 flex justify-end items-center mr-2 bg-base-200 pr-2");

    html! {
        <label class={classes!(class.clone())}>
            {props.children.clone()}
        </label>        
    }
}