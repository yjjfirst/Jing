use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or("text".to_string())]
    pub input_type: String,
    pub id: String,
    pub value: String,
    #[prop_or(classes!("col-span-2"))]
    pub classes: Classes,
    #[prop_or(false)]
    pub hidden: bool,
    #[prop_or(false)]
    pub disabled: bool
}

#[function_component]
pub fn Input(props: &Props) -> Html {    
    let input_type = props
        .input_type
        .clone();

    let input_value = props
        .value
        .clone();

    let id = props
        .id
        .clone();

    let name = id.clone();
    let mut hidden = classes![""];

    if props.hidden == true {
        hidden.push("hidden");
    }

    let input_class = classes!("input", "input-bordered", "block", "w-full", hidden, props.classes.clone());
    html! {
        <input
            type={input_type}
            placeholder=""
            value={input_value}
            id={id.clone()}
            name={name}
            class={input_class}
            disabled={props.disabled}/>
    }
}

