use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub input_type: String,
    pub id: String,
    pub label: String,
    pub name: String,
    pub value: String,
    pub input_ref: NodeRef,
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

    html! {
        <div>
            <label class="label">
            <span 
                for={id.clone()} 
                class="label-text">
                {props.label.clone()}
            </span>
            </label>
            <input
                type={input_type}
                placeholder=""
                value={input_value}
                id={id.clone()}
                name={props.name.clone()}
                class="input input-bordered block w-full"
                ref={props.input_ref.clone()}
            />
        </div>        
    }
}