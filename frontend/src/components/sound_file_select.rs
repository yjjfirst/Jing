use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use crate::store::Store;
use yewdux::prelude::*;
use crate::services::Service;
use super::label::Label;
use crate::utils::string::capitalize;
use crate::services::sound_file::SoundFile;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
    #[prop_or(classes!("w-80"))]
    pub label_width: Classes,
    pub sound_file_id: usize
}
#[function_component]
pub fn SoundFileSelect(props: &Props) -> Html {
    let id = props.id.clone();
    let sound_file_id = props.sound_file_id;
    let name= id.clone();
    let input_ref: NodeRef = use_node_ref();

    let label = name.replace("_", " ").replace("-", " ");
    let label = capitalize(&label);
    let label_class: Classes = props.label_width.clone();

    let(store,_) = use_store::<Store>();
    let sound_files: UseStateHandle<Vec<SoundFile>> = use_state(||vec![]);
    {
        let sound_files = sound_files.clone();
        use_effect_with((), move|_| {
            let store = store.clone();
            let sound_files = sound_files.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_files: Vec<SoundFile> = 
                    Service::index("/sound-file", store.selected_domain.clone())
                        .await
                        .unwrap();
                sound_files.set(fetched_files);
            });        
        });
    }

    let options: Vec<Html> = sound_files
        .iter()
        .map(|s|{
            html!{
                if (s.id) == props.sound_file_id {
                    <option value={s.name.clone()} selected=true >{s.name.clone()}</option>
                } else {
                    <option value={s.name.clone()}>{s.name.clone()}</option>
                }
            }
        })
        .collect();

    let on_changed = {
        let input_ref = input_ref.clone();
        let sound_files = sound_files.clone();
        Callback::from(move |e: Event| {
            let select = e.target_dyn_into::<HtmlSelectElement>();
            if let Some(select) = select {
                let e = input_ref.cast::<HtmlInputElement>().unwrap();
                for s in sound_files.iter() {
                    if s.name == select.value() {
                        e.set_value(&s.id.to_string());
                    }
                }
            };
        })
    };

    html! {
        <div class={classes!("w-full", "px-3", "mb-6", "md:mb-0")}>
            <input 
                id={id.clone()} 
                ref={input_ref} 
                hidden={true}
                name={name} 
                value={sound_file_id.to_string()}/>
            <div class="flex mb-1">
                if label != "" {
                    <Label class={label_class}>
                        <span 
                            for={id.clone()} 
                            class="label-text">
                            {label}
                        </span>
                    </Label>
                }
                <select class="select select-bordered w-full" 
                    onchange={on_changed}
                >
                {options}
                </select>
            </div>
        </div>
    }   
}
