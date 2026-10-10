pub mod model;

use web_sys::{EventTarget, FormData, SubmitEvent, HtmlFormElement, HtmlDialogElement};
use wasm_bindgen::JsCast;

use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yew_icons::{Icon, IconData};

use crate::components::file_input::FileInput;
use crate::components::action_buttons::ActionButtons;
use crate::components::header::Header;
use crate::store::{alert_info, alert_error, Store};
use crate::components::dialog::Dialog;
use model::Recording;
use crate::models::Service;

#[derive(Clone, PartialEq, Properties)]
pub struct RecordingProps {
    pub recording: Recording,
    pub ondel: Callback<usize>
}

#[derive(Clone, PartialEq, Properties)]
pub struct RecordingDetailProps {
    pub id: usize,
}

#[derive(Clone, Routable, PartialEq)]
pub enum RecordingRoute {
    #[at("/recording")]
    Index,
    #[at("/recording/:id")]
    Get {id: usize},
}
#[function_component]
pub fn RecordingListItem(props: &RecordingProps) -> Html {
    let recording = props.recording.clone();
    let nav = use_navigator().unwrap();
    let dialog_ref: NodeRef = use_node_ref();
    let loc: Location = use_location().unwrap().clone();
    let (store, dispatch) = use_store::<Store>();
    let ondel = props.ondel.clone();

    let onedit: Callback<MouseEvent> = Callback::from(move|_e|{
        nav.push(&RecordingRoute::Get {id: recording.id});
    });

    let handle_confirm: Callback<bool> = Callback::from(move|_e: bool|{
        let loc = loc.clone();
        let store = store.clone();
        let ondel = ondel.clone();
        let dispatch = dispatch.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let path = format!("{}/{}", loc.path(), recording.id);
            match Service::delete(&path, store.clone().selected_domain_id)
                .await
            {
                Ok(_) => ondel.emit(recording.id),
                Err(_) => {
                    alert_error("Delete recording failed".to_string(), dispatch);
                }
            }
        })
    });

    let handle_del = {
        let dialog_ref = dialog_ref.clone();
        Callback::from(move|_e: MouseEvent|{
            let dialog_ref = dialog_ref.clone();
            let d = dialog_ref.cast::<HtmlDialogElement>().unwrap();
            d.show_modal().unwrap();
        })
    };

    html! {
        <tr>
            <th>{props.recording.name.clone()}</th>
            <th>{props.recording.description.clone()}</th>
            <th class="flex justify-end">
                <div class="mr-1">
                    <div onclick={onedit} class="btn btn-square btn-outline btn-sm">
                        <Icon data={IconData::LUCIDE_EDIT}/>
                    </div>
                </div>
                <div>
                    <div onclick={handle_del} class="btn btn-square btn-outline btn-sm">
                        <Icon data={IconData::LUCIDE_TRASH}/>
                    </div>
                </div>
            </th>
            <Dialog
                d_ref = {dialog_ref.clone()}
                title={"Warning!"}
                contents={format!("Are you sure to delete recording: {}", recording.name.clone())}
                onconfirm={handle_confirm}
                >
            </Dialog>
        </tr>
    }
}

#[function_component]
pub fn RecordingList() -> Html {
    let (store,_) = use_store::<Store>();
    let recordings: UseStateHandle<Vec<Recording>> = use_state(||vec![]);
    let loc = use_location().unwrap().clone();
    let nav = use_navigator().unwrap();

    {
        let recordings = recordings.clone();
        use_effect_with((), move|_| {
            let store = store.clone();
            let recordings = recordings.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched: Vec<Recording> =
                    Service::index(loc.path(), store.selected_domain_id.clone())
                        .await
                        .unwrap();
                recordings.set(fetched);
            });
        });
    }

    let onadd = Callback::from(move|_e: MouseEvent|{
        nav.push(&RecordingRoute::Get { id: 0 });
    });

    let handle_del = {
        let recordings = recordings.clone();
        Callback::from(move| id: usize|{
            let recordings = recordings.clone();
            let filtered: Vec<Recording> = recordings
                .iter()
                .filter(|f|f.id != id)
                .map(|f|f.clone())
                .collect();

            recordings.set(filtered);
        })
    };

    html! {
        <div class="grow mr-2">
            <Header title="System -> Recordings"></Header>
            <div class="divider my-1"></div>
            <table class="table table-zebra">
                <thead>
                    <tr>
                        <th>{"Name"}</th>
                        <th>{"Description"}</th>
                    </tr>
                </thead>
                <tbody>
                {recordings.iter().map(move|s|{
                    html!{
                        <RecordingListItem recording={Recording {..s.clone()}} ondel={handle_del.clone()}/>
                    }
                }).collect::<Html>()}
                </tbody>
            </table>
            <div class="flex flex-row-reverse pr-4">
                <div onclick={onadd} class="btn btn-square btn-outline btn-sm" >
                    <Icon data={IconData::LUCIDE_PLUS}/>
                </div>
            </div>
        </div>

    }
}

#[function_component]
pub fn RecordingDetail(props: &RecordingDetailProps) -> Html {
    let nav = use_navigator().unwrap();
    let loc = use_location().unwrap();
    let (store,dispatch) = use_store::<Store>();
    let id = props.id;
    let recording = use_state(||{
        Recording {
            id: 0,
            name: "".to_string(),
            domain_id: 0,
            description: "".to_string(),
        }
    });

    {
        let recording = recording.clone();
        let loc = loc.clone();
        let store = store.clone();
        use_effect_with((), move |_| {
            let recording = recording.clone();
            let loc = loc.clone();
            let store = store.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_out =
                    Service::get(loc.path(), store.selected_domain_id)
                        .await
                        .unwrap();
                recording.set(fetched_out);
            });
        });
    }

    let form_oncancel = {
        let nav = nav.clone();
        Callback::from(move|_| {
            nav.push(&RecordingRoute::Index);
        })
    };

    let form_onsubmit = {
        let loc = loc.clone();
        let store = store.clone();
        let dispatch = dispatch.clone();
        Callback::from(move|event: SubmitEvent| {
            let target: Option<EventTarget> = event.target();
            let form = target.unwrap().dyn_into::<HtmlFormElement>().unwrap();
            let form_data = FormData::new_with_form(&form).unwrap();
            let dispatch = dispatch.clone();
            let loc = loc.clone();
            let nav = nav.clone();
            let store = store.clone();

            event.prevent_default();

            let recording = Recording {
                id: form_data.get("id").as_string().unwrap().parse::<usize>().unwrap(),
                name: "".to_string(),
                domain_id: 0,
                description: form_data.get("description").as_string().unwrap(),
            };

            if id == 0 {
                wasm_bindgen_futures::spawn_local(async move {
                    let dispatch = dispatch.clone();
                    let loc = loc.clone();
                    match Service::post_form(loc.path(), store.selected_domain_id, form_data)
                        .await {
                            Ok(_) => {
                                alert_info("Create recording successfully".to_string(), dispatch);
                                nav.push(&RecordingRoute::Index);
                            }
                            Err(_) => {
                                alert_error("Create recording failed".to_string(), dispatch);
                            }
                        }
                });
            } else {
                wasm_bindgen_futures::spawn_local(async move {
                    let dispatch = dispatch.clone();
                    let loc = loc.clone();

                    match Service::patch(loc.path(), store.selected_domain_id, recording)
                        .await {
                            Ok(_) => {
                                alert_info("Update recording successfully".to_string(), dispatch);
                                nav.push(&RecordingRoute::Index);
                            }
                            Err(_) => {
                                alert_error("Update recording failed".to_string(), dispatch);
                            }
                        }
                });
            }
        })
    };

    html!{
        <div class="grow mr-2">
            <Header title= {format!("Recording: {}", recording.name)}></Header>
            <div class="divider my-1"></div>
            <form class="w-full" onsubmit={form_onsubmit} method="POST">
            <div class="grid grid-cols-3 gap-1">
                <input value={recording.id.to_string()} name="id" hidden=true />
                <input value={store.selected_domain_id.to_string()} name="domain_id" hidden=true />
                <label class="pbx-label">{"Name"}</label>
                if id == 0 {
                    <FileInput
                    id="file_name"
                    value=""
                    input_type="file"/>
                } else {
                    <input class="pbx-input" value={recording.name.clone()} name="name" disabled=true />
                }
                <label class="pbx-label">{"Description"}</label>
                <input class="pbx-input" value={recording.description.clone()} name="description" />
                </div>
                <ActionButtons oncancel={form_oncancel}/>
            </form>
        </div>
    }
}

pub fn recording_switch(route: RecordingRoute) -> Html {
    match route {
        RecordingRoute::Index => html!{<RecordingList />},
        RecordingRoute::Get {id} => html !{<RecordingDetail id={id}/>}
    }
}