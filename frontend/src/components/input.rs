use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub input_type: Option<String>,
    pub id: Option<String>,
    pub label: String,
    pub name: String,
    pub value: Option<String>,
    pub input_ref: NodeRef,
    pub on_changed: Option<Callback<String>>,
    pub on_blur: Option<Callback<(String, String)>>
}

#[function_component]
pub fn Input(props: &Props) -> Html {    
    let input_type = props
        .input_type
        .clone()
        .unwrap_or_else(||"text".to_string());

    let input_value = props
        .value
        .clone()
        .unwrap_or_else(|| "".to_string());

    let id = props
        .id
        .clone()
        .unwrap_or_else(|| "".to_string());

    let on_changed = {
        let on_changed = props.on_changed.clone();
        Callback::from(move|e: Event|{
            let target = e.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();
            if let Some(on) = on_changed.clone() {
                on.emit(value);
            }

        })
    };

    let on_blur = {
        let on_blur = props.on_blur.clone();
        let name = props.name.clone();
        Callback::from(move|e: FocusEvent |{
            let name = name.clone();
            let target = e.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();
            if let Some(on) = on_blur.clone() {
                on.emit((name, value));
            }
        })
    };

    html! {
        <div>
            <label 
                for={id.clone()} 
                class="block uppercase tracking-wide text-xs mb-2">
                {props.label.clone()}
            </label>
            <input
                type={input_type}
                placeholder=""
                value={input_value}
                id={id.clone()}
                name={props.name.clone()}
                class="appearance-none block w-full rounded-md py-2 px-4 mb-3 leading-tight focus:outline outline-1"
                ref={props.input_ref.clone()}
                onchange={on_changed}
                onblur={on_blur}
            />
      </div>        
    }
}