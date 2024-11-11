use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use web_sys::{Event, HtmlInputElement};
use gloo_file::File;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or("text".to_string())]
    pub input_type: String,
    pub id: String,
    pub value: String,
    #[prop_or(false)]
    pub hidden: bool,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or(classes!("col-span-2"))]
    pub classes: Classes,    
}

#[function_component]
pub fn FileInput(props: &Props) -> Html {    
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

    let input_class = classes!("file-input", 
        "file-input-bordered", 
        "w-full", 
        props.classes.clone());

    let onchange = Callback::from( move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let files = input.files();

        if let Some(files) = files {
            let files = js_sys::try_iter(&files)
                .unwrap()
                .unwrap()
                .map(|v|web_sys::File::from(v.unwrap()))
                .map(File::from);

            for file in files {
                wasm_bindgen_futures::spawn_local(async move {
                let _data = gloo_file::futures::read_as_bytes(&file)
                    .await
                    .expect_throw("read file");

                })
            }
        }
    });

    html! {
        <input
            type={input_type}
            placeholder=""
            value={input_value}
            id={id.clone()}
            name={name}
            class={input_class}
            onchange={onchange}
            disabled={props.disabled}/>
    }
}

