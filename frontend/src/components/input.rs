use yew::prelude::*;
use super::label::Label;
use crate::utils::string::capitalize;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or("text".to_string())]
    pub input_type: String,
    pub id: String,
    pub value: String,
    #[prop_or(classes!("w-80"))]
    pub label_width: Classes,
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
    let label = name.replace("_", " ").replace("-", " ");
    let label = capitalize(&label);
    let mut hidden = classes![""];

    if props.hidden == true {
        hidden.push("hidden");
    }
    
    let label_class: Classes = props.label_width.clone();

    html! {
        <div class={classes!("w-full", "px-3", "mb-6", "md:mb-0", hidden.clone())}>
        <div class="flex mb-1 ">
            if label != "" {
                <Label class={label_class}>
                    <span 
                        for={id.clone()} 
                        class="label-text">
                        {label}
                    </span>
                </Label>
            }
            <input
                type={input_type}
                placeholder=""
                value={input_value}
                id={id.clone()}
                name={name}
                class="input input-bordered block w-full"
                disabled={props.disabled}/>
        </div>
        </div>       
    }
}

