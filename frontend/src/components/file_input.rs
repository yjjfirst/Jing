use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use super::label::Label;
use crate::utils::string::capitalize;
use web_sys::{Event, HtmlInputElement};
use gloo_file::File;

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
    let label = name.replace("_", " ").replace("-", " ");
    let label = capitalize(&label);
    let mut hidden = classes![""];

    if props.hidden == true {
        hidden.push("hidden");
    }

    let input_class = classes!("file-input", "file-input-bordered", "w-full");
    let label_class: Classes = props.label_width.clone();

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
                    class={input_class}
                    onchange={onchange}
                    disabled={props.disabled}/>
            </div>
        </div>       
    }
}

