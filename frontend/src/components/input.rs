use yew::prelude::*;
use super::label::Label;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub input_type: String,
    pub id: String,
    #[prop_or_default]
    pub label: String,
    pub name: String,
    pub value: String,
    #[prop_or_default]
    pub label_width: Classes,
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
    let label_class: Classes = props.label_width.clone();

    let label = props.label.clone();
    html! {
        <div class="flex mb-1">
            if label != "" {
                <Label class={label_class}>
                    <span 
                        for={id.clone()} 
                        class="label-text">
                        {props.label.clone()}
                    </span>
                </Label>
            }
            <input
                type={input_type}
                placeholder=""
                value={input_value}
                id={id.clone()}
                name={props.name.clone()}
                class="input input-bordered block w-full"
            />
        </div>        
    }
}
