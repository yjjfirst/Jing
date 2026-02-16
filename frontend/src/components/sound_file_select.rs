use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use crate::store::Store;
use yewdux::prelude::*;
use crate::models::Service;
use crate::pages::sound_file::model::SoundFile;

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
    let select_ref: NodeRef = use_node_ref();
    let input_ref: NodeRef = use_node_ref();
    let(store,_) = use_store::<Store>();
    let sound_files: UseStateHandle<Vec<SoundFile>> = use_state(||vec![]);
    {
        let sound_files = sound_files.clone();
        let select_ref = select_ref.clone();
        use_effect_with((), move|_| {
            let select_ref = select_ref.clone();
            let store = store.clone();
            let sound_files = sound_files.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_files: Vec<SoundFile> = 
                    Service::index("/sound-file", store.selected_domain.clone())
                        .await
                        .unwrap();
                sound_files.set(fetched_files);
                let s = select_ref.cast::<HtmlSelectElement>().unwrap();
                s.set_disabled(false);
            });        
        });
    }

    let mut options: Vec<Html> = sound_files
        .iter()
        .map(|s|{
            html!{
                if (s.id) == props.sound_file_id {
                    <option value={s.name.clone()} selected=true>{s.name.clone()}</option>
                } else {
                    <option value={s.name.clone()}>{s.name.clone()}</option>
                }
            }
        })
        .collect();

    if  props.sound_file_id == 0 {
        options.push(
                html!{
                    <option value={"".to_string()}
                        selected=true
                        disabled=true
                        hidden=true>{""}
                    </option>
                });
    }
        
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
        <div class={classes!("w-full", "mb-6", "md:mb-0", "col-span-2")}>
            <input 
                id={id.clone()} 
                ref={input_ref} 
                hidden={true}
                name={name} 
                value={sound_file_id.to_string()}/>
            <div class="flex mb-1">
                <select class="select select-bordered w-full"
                    onchange={on_changed}
                    disabled=true
                    ref={select_ref}
                >
                {options}
                </select>
            </div>
        </div>
    }   
}
