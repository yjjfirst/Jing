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
            <span 
                for={id.clone()} 
                class="block uppercase tracking-wide text-xs mb-2">
                {props.label.clone()}
            </span>
            <input
                type={input_type}
                placeholder=""
                value={input_value}
                id={id.clone()}
                name={props.name.clone()}
                class="appearance-none block w-full text-sm rounded-md py-2 px-4 mb-3 leading-tight focus:outline outline-1"
                ref={props.input_ref.clone()}
            />
        </div>        
    }
}