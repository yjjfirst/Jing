pub mod model;

use wasm_bindgen::JsCast;
use web_sys::{FormData, SubmitEvent, HtmlFormElement, HtmlDialogElement};
use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yew_icons::{Icon, IconData};

use crate::components::header::Header;
use crate::components::dialog::Dialog;
use crate::components::select_recording::SelectRecording;
use crate::components::action_buttons::ActionButtons;

use crate::store::{alert_info, alert_error, Store};
use crate::models::Service;
use model::{Announcement};
use crate::pages::recording::model::Recording;

#[derive(Clone, Routable, PartialEq)]
pub enum AnnouncementRoute {
    #[at("/announcement")]
    Index,
    #[at("/announcement/:id")]
    Get {id: usize},
}

#[derive(Clone, PartialEq, Properties)]
pub struct AnnouncementDetailProps {
    id: usize,
}

#[derive(Clone, PartialEq, Properties)]
pub struct AnnouncementListItemProps {
    pub id: usize,
    pub name: String,
    pub recording: String,
    pub exten: String,
    pub ondel: Callback<usize>
}

#[function_component]
pub fn AnnouncementListItem(props: &AnnouncementListItemProps) -> Html {
    let nav = use_navigator().unwrap();
    let dialog_ref: NodeRef = use_node_ref();
    let dd_ref = dialog_ref.clone();
    let loc: Location = use_location().unwrap().clone();
    let (store,_) = use_store::<Store>();
    let ondel = props.ondel.clone();
    let id = props.id;

    let onedit: Callback<MouseEvent> = Callback::from(move |_e|{
        nav.push(&AnnouncementRoute::Get {id: id});
    });

    let handle_confirm: Callback<bool> = Callback::from(move|_e: bool|{
        let loc = loc.clone();
        let store = store.clone();
        let ondel = ondel.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let path = format!("{}/{}", loc.path(), id);
            Service::delete(&path, store.clone().selected_domain_id)
                .await
                .unwrap();
            ondel.emit(id);
        });
    });

    let ondel: Callback<MouseEvent> = Callback::from(move |_e| {
        let d = dd_ref.cast::<HtmlDialogElement>().unwrap();
        d.show_modal().unwrap();
    });

    html!{
        <tr>
            <th>{props.exten.clone()}</th>
            <th>{props.name.clone()}</th>
            <th>{props.recording.clone()}</th>
            <th class="flex justify-end">
                <div class="mr-1">
                    <div onclick={onedit} class="btn btn-square btn-outline btn-sm">
                        <Icon data={IconData::LUCIDE_EDIT}/>
                    </div>
                </div>
                <div>
                    <div onclick={ondel} class="btn btn-square btn-outline btn-sm">
                        <Icon data={IconData::LUCIDE_TRASH}/>
                    </div>
                </div>
            </th>
            <Dialog
                d_ref = {dialog_ref}
                title={"Warning!"}
                contents={format!("Are you sure to delete announcement: {}?", props.name.clone())}
                onconfirm={handle_confirm}
                >
            </Dialog>

        </tr>
    }
}

#[function_component]
pub fn AnnouncementList() -> Html {
    let loc = use_location().unwrap().clone();
    let nav = use_navigator().unwrap();
    let (store,_) = use_store::<Store>();
    let announcements = use_state(||vec![]);
    {
        let announcements = announcements.clone();
        use_effect_with((), move|_|{
            let announcements = announcements.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched: Vec<Announcement> =
                    Service::index(loc.path(), store.selected_domain_id.clone())
                        .await
                        .unwrap();
                announcements.set(fetched);
            });
        });
    }

    let ondel: Callback<usize> = {
        let announcements = announcements.clone();
        Callback::from(move|id: usize|{
            let filtered: Vec<Announcement> = announcements
                .iter()
                .filter(|a|a.id != id)
                .map(|a|a.clone())
                .collect();
            announcements.set(filtered);
        })
    };

    let announcements_html: Vec<Html> = announcements
        .iter()
        .map(|a| {
            html! {
                <AnnouncementListItem
                    name={a.name.clone()}
                    exten={a.exten.clone()}
                    recording={a.recording.name.clone()}
                    id={a.id}
                    ondel={ondel.clone()}
                />
            }
        })
        .collect();

    let onadd: Callback<MouseEvent> = Callback::from(move|_e: MouseEvent|{
        nav.push(&AnnouncementRoute::Get {id: 0});
    });

    html! {
        <div class="grow mr-2">
            <Header title="Application -> Announcement"></Header>
            <div class="divider my-1"></div>
            <table class="table table-zebra">
                <thead>
                    <tr>
                        <th>{"Extension"}</th>
                        <th>{"Name"}</th>
                        <th>{"Recording"}</th>
                    </tr>
                    {announcements_html}
                </thead>
                <tbody>

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
pub fn AnnouncementDetails(props: &AnnouncementDetailProps) -> Html {
    let id = props.id;
    let loc = use_location().unwrap();
    let nav = use_navigator().unwrap();
    let announcement = use_state(||Announcement::new());
    let(store, dispatch) = use_store::<Store>();

    {
        let announcement = announcement.clone();
        let loc = loc.clone();
        let store = store.clone();
        use_effect_with((), move |_|{
            let s = announcement.clone();
            let loc = loc.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_announcements =
                    Service::get(loc.path(), store.selected_domain_id)
                        .await
                        .unwrap();
                s.set(fetched_announcements);
            });
        });
    }

    let form_oncancel = {
        let nav = nav.clone();
        Callback::from(move|_| {
            nav.push(&AnnouncementRoute::Index);
        })
    };

    let form_onsubmit = {
        let store = store.clone();
        let dispatch = dispatch.clone();
        let announcement = announcement.clone();
        let nav = nav.clone();

        Callback::from(move|e: SubmitEvent| {
            let dispatch = dispatch.clone();
            let loc = loc.clone();
            let store = store.clone();
            let nav = nav.clone();
            e.prevent_default();


            let form_data = FormData::new_with_form(
                &e
                    .target()
                    .unwrap()
                    .dyn_into::<HtmlFormElement>()
                    .unwrap()).unwrap();
            let mut recording = Recording::new();
            recording.id = form_data
                .get("recording")
                .as_string()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let s = Announcement {
                id: announcement.id,
                name: form_data.get("name").as_string().unwrap(),
                domain_id: store.selected_domain_id,
                recording,
                exten: form_data
                    .get("exten")
                    .as_string()
                    .unwrap()
            };

            wasm_bindgen_futures::spawn_local(async move {
                let dispatch = dispatch.clone();
                let loc = loc.clone();

                match Service::post(loc.path(), store.selected_domain_id, s).await {
                    Ok(_) => {
                        alert_info("Update announcement successfully.".to_string(), dispatch);
                    }
                    Err(_) => {
                        alert_error("Update announcement failed.".to_string(), dispatch);
                    }
                }
                nav.push(&AnnouncementRoute::Index);
            });

        })
    };

    html!{
        <div class="grow mr-2">
            <Header title= {format!("Announcement: {}", announcement.exten.clone())}></Header>
            <div class="divider my-1"></div>
            <form class="w-full" onsubmit={form_onsubmit}>
            <div class="grid grid-cols-3 gap-1">
                <label class="pbx-label" hidden = {id != 0}>{"Extension"}</label>
                <input class="pbx-input"
                    value={announcement.exten.clone()}
                    name="exten"
                    hidden = {id != 0}
                />
                <label class="pbx-label">{"Name"}</label>
                <input class="pbx-input"
                    value={announcement.name.clone()}
                    name="name"
                />
                <label class="pbx-label">{"Recording"}</label>
                <div class="col-span-2">
                    <SelectRecording name="recording" recording_id={announcement.recording.id}/>
                </div>
            </div>
            <ActionButtons oncancel={form_oncancel}/>
            </form>
        </div>
    }
}

pub fn announcement_switch(route: AnnouncementRoute) -> Html {
    match route {
        AnnouncementRoute::Index => html!{<AnnouncementList />},
        AnnouncementRoute::Get { id } => html !{<AnnouncementDetails id={id}/>}
    }
}
