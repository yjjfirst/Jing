use yew::prelude::*;
use crate::store::Store;
use yewdux::prelude::*;
use crate::models::Service;
use crate::pages::sound_file::model::SoundFile;
use crate::components::select_id::SelectId;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
    #[prop_or(classes!("w-80"))]
    pub label_width: Classes,
    pub sound_file_id: usize
}
#[function_component]
pub fn SelectSoundFile(props: &Props) -> Html {
    let id = props.id.clone();
    let sound_file_id = props.sound_file_id;
    let(store,_) = use_store::<Store>();
    let sound_files: UseStateHandle<Vec<SoundFile>> = use_state(||vec![]);
    {
        let sound_files = sound_files.clone();
        use_effect_with((), move|_| {
            let store = store.clone();
            let sound_files = sound_files.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_files: Vec<SoundFile> =
                    Service::index("/sound-file", store.selected_domain_id.clone())
                        .await
                        .unwrap();
                sound_files.set(fetched_files);
            });
        });
    }
    html! {
        <SelectId
            id={id.clone()}
            options={
                sound_files
                    .iter()
                    .map(|s|{s.name.clone()})
                    .collect::<Vec<String>>()
            }
            options_id={
                sound_files
                    .iter()
                    .map(|s|{s.id.clone()})
                    .collect::<Vec<usize>>()
            }
            selected={sound_file_id}
        >
        </SelectId>
    }
}
