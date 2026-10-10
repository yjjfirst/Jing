use yew::prelude::*;
use crate::store::Store;
use yewdux::prelude::*;
use crate::models::Service;
use crate::pages::recording::model::Recording;
use crate::components::select_id::SelectId;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    #[prop_or(classes!("w-80"))]
    pub label_width: Classes,
    pub recording_id: usize
}
#[function_component]
pub fn SelectRecording(props: &Props) -> Html {
    let name = props.name.clone();
    let recording_id = props.recording_id;
    let(store,_) = use_store::<Store>();
    let recordings: UseStateHandle<Vec<Recording>> = use_state(||vec![]);
    {
        let recordings = recordings.clone();
        use_effect_with((), move|_| {
            let store = store.clone();
            let recordings = recordings.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_recordings: Vec<Recording> =
                    Service::index("/recording", store.selected_domain_id.clone())
                        .await
                        .unwrap();
                recordings.set(fetched_recordings);
            });
        });
    }
    html! {
        <SelectId
            name={name.clone()}
            options={
                recordings
                    .iter()
                    .map(|r|{r.name.clone()})
                    .collect::<Vec<String>>()
            }
            options_id={
                recordings
                    .iter()
                    .map(|r|{r.id.clone()})
                    .collect::<Vec<usize>>()
            }
            selected={recording_id}
        >
        </SelectId>
    }
}
