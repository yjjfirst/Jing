use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LabelProps {
    #[prop_or(classes!(""))]
    pub class: Classes,
    pub children: Html,
    #[prop_or(false)]
    pub hidden: bool,    
}

#[function_component]
pub fn Label(props: &LabelProps) -> Html {
    let mut class = props.class.clone();
    class.push("label-text shrink-0 flex justify-end items-center pr-2 bg-base-200");

    if props.hidden == true {
        class.push("hidden");
    }
    
    html! {
        <label class={classes!(class.clone())}>
            {props.children.clone()}
        </label>        
    }
}