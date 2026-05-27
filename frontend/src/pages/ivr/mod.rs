pub mod model;

use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::{FormData, SubmitEvent, HtmlDialogElement};
use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yew_icons::{Icon, IconId};

use crate::components::header::Header;
use crate::components::dialog::Dialog;
use crate::components::input::Input;
use crate::components::sound_file_select::SoundFileSelect;
use crate::components::exten_select::ExtenionSelect;
use crate::components::label::Label;

use crate::components::action_buttons::ActionButtons;

use model::{IvrEntry, IvrAttr, Ivr};
use crate::store::{alert_info, alert_error, Store};
use crate::models::Service;

#[derive(Clone, Routable, PartialEq)]
pub enum IvrRoute {
    #[at("/ivr")]
    Index,
    #[at("/ivr/:id")]
    Get {id: usize},
}

#[derive(Clone, PartialEq, Properties)]
pub struct IvrDetailProps {
    pub id: usize,
}

#[derive(Clone, PartialEq, Properties)]
pub struct IvrListItemProps {
    pub id: usize,
    pub exten: String,
    pub name: String,
    pub ondel: Callback<usize>
}

#[derive(Clone, PartialEq, Properties)]
pub struct IvrEntryProps {
    pub digits: String,
    pub exten: String,
}

#[function_component]
pub fn IvrListItem(props: &IvrListItemProps) -> Html {
    let nav = use_navigator().unwrap();
    let dialog_ref: NodeRef = use_node_ref();
    let loc: Location = use_location().unwrap();
    let (store,_) = use_store::<Store>();
    let ondel = props.ondel.clone();
    let ivr_id = props.id;

    let onedit: Callback<MouseEvent> = {
        let props = props.clone();
        Callback::from(move |_e|{
            let nav = nav.clone();
            nav.push(&IvrRoute::Get {id: props.id});
        })
    };
    let onconfirm: Callback<bool> = Callback::from(move|_e: bool|{
        let loc = loc.clone();
        let store = store.clone();
        let ondel = ondel.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let path = format!("{}/{}", loc.path(), ivr_id);
            Service::delete(&path, store.clone().selected_domain_id)
                .await
                .unwrap();
            ondel.emit(ivr_id);
        });
    });

    let ondel: Callback<MouseEvent> = {
        let dialog_ref = dialog_ref.clone();
        Callback::from(move |_e| {
            let d = dialog_ref.cast::<HtmlDialogElement>().unwrap();
                d.show_modal().unwrap();
        })
    };

    html! {
        <tr>
            <th>{props.exten.clone()}</th>
            <th>{props.name.clone()}</th>
            <th class="flex justify-end">
                <div class="mr-1">
                    <div onclick={onedit} class="btn btn-square btn-outline btn-sm">
                        <Icon icon_id={IconId::LucideEdit}/>
                    </div>
                </div>
                <div>
                    <div onclick={ondel} class="btn btn-square btn-outline btn-sm">
                        <Icon icon_id={IconId::LucideTrash}/>
                    </div>
                </div>
            </th>
            <Dialog
                d_ref = {dialog_ref}
                title={"Warning!"}
                contents={format!("Are you sure to delete IVR: {}?", props.exten.clone())}
                {onconfirm}
                >
            </Dialog>
        </tr>
    }
}

#[function_component]
pub fn IvrList() -> Html {
    let nav = use_navigator().unwrap();
    let loc = use_location().unwrap().clone();
    let (store,_) = use_store::<Store>();

    let ivrs = use_state(||vec![]);

    {
        let ivrs = ivrs.clone();
        use_effect_with((), move|_|{
            let ivrs = ivrs.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_ivrs: Vec<Ivr> =
                    Service::index(loc.path(), store.selected_domain_id.clone())
                        .await
                        .unwrap();
                ivrs.set(fetched_ivrs);
            });
        });
    }
    let onadd: Callback<MouseEvent> = Callback::from(move|_e: MouseEvent|{
        nav.push(&IvrRoute::Get {id: 0});
    });

    let ondel: Callback<usize> = {
        let ivrs = ivrs.clone();
        Callback::from(move|id: usize|{
            let ivrs = ivrs.clone();
            let filtered = ivrs
                .iter()
                .filter(|i| id != i.id)
                .map(|i| i.clone())
                .collect();
            ivrs.set(filtered);
        })
    };

    let list_items: Vec<Html> = ivrs
        .iter()
        .map(|i|{
            html! {
                <IvrListItem
                    id={i.id}
                    exten={i.exten.clone()}
                    name={i.name.clone()}
                    ondel={ondel.clone()}/>
            }
        })
        .collect();

    html!{
        <div class="grow mr-2">
            <Header title="Application -> IVR"></Header>
            <div class="divider my-1"></div>
            <table class="table table-zebra">
                <thead>
                    <tr>
                        <th>{"Extension"}</th>
                        <th>{"Name"}</th>
                    </tr>
                </thead>
                <tbody>
                    {list_items}
                </tbody>
            </table>
            <div class="flex flex-row-reverse pr-4">
                <div onclick={onadd} class="btn btn-square btn-outline btn-sm" >
                    <Icon icon_id={IconId::LucidePlus}/>
                </div>
            </div>
        </div>
    }
}

#[function_component]
pub fn IvrEntryComponent(props: &IvrEntryProps) -> Html {
    html!{
        <div class="flex">
            <Input id="entry" value={props.digits.clone()}></Input>
            <ExtenionSelect id="destination" value={props.exten.clone()}/>
        </div>
    }
}

#[function_component]
pub fn IvrDetails(props: &IvrDetailProps) -> Html {
    let nav = use_navigator().unwrap();
    let ivr = use_state(||Ivr::new());
    let loc = use_location().unwrap();
    let(store, dispatch) = use_store::<Store>();
    let form_oncancel = {
        let nav = nav.clone();
        Callback::from(move|_| {
            nav.push(&IvrRoute::Index);
        })
    };
    let store_cloned = store.clone();

    {
        let ivr = ivr.clone();
        let loc = loc.clone();
        let store = store.clone();
        use_effect_with((), move |_| {
            let ivr = ivr.clone();
            let loc = loc.clone();
            let store = store.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_ivr =
                    Service::get(loc.path(), store.selected_domain_id)
                        .await
                        .unwrap();
                ivr.set(fetched_ivr);
            });
        });
    }

    let label = |id:&str| {
        html!{
            <Label>{id}</Label>
        }
    };

    let input =  |id :&str, value: &str| {
        html!{
            <Input id={id.to_string()} value={value.to_string()}></Input>
        }
    };

    let sound_file_select = |id: &str, sound_file_id: usize| {
        html!{
            <SoundFileSelect
                id={id.to_string()}
                sound_file_id={sound_file_id}>
            </SoundFileSelect>
        }
    };

    let attr_htmls: Vec<Html> =
        vec![
            label("Greet Long"),
            sound_file_select("greet-long", IvrAttr::get("greet-long", &ivr.attrs).parse::<usize>().unwrap_or(0)),
            label("Greet Short"),
            sound_file_select("greet-short", IvrAttr::get("greet-short", &ivr.attrs).parse::<usize>().unwrap_or(0)),
            label("Invalid Sound"),
            sound_file_select("invalid-sound", IvrAttr::get("invalid-sound", &ivr.attrs).parse::<usize>().unwrap_or(0)),
            label("Exit Sound"),
            sound_file_select("exit-sound", IvrAttr::get("exit-sound", &ivr.attrs).parse::<usize>().unwrap_or(0)),
            label("Timeout"),
            input("timeout", &IvrAttr::get("timeout", &ivr.attrs)),
            label("Inter Digit Timeout"),
            input("inter-digit-timeout", &IvrAttr::get("inter-digit-timeout", &ivr.attrs)),
            label("Max Failures"),
            input("max-failures", &IvrAttr::get("max-failures", &ivr.attrs)),
            label("Max Timeouts"),
            input("max-timeouts", &IvrAttr::get("max-timeouts", &ivr.attrs)),
            label("Digit Len"),
            input("digit-len", &IvrAttr::get("digit-len", &ivr.attrs)),
            label("Confirm attempts"),
            input("confirm-attempts", &IvrAttr::get("confirm-attempts", &ivr.attrs)),
        ];


    let entries_html: Vec<Html> = ivr.entries.iter().map(|e|{
        html!{
            <IvrEntryComponent digits={e.digits.clone()} exten={e.dest_exten.clone()} />
        }
    }).collect();

    let add_entry = {
        let ivr = ivr.clone();
        Callback::from(move |_e: MouseEvent|{
            let ivr = ivr.clone();
            let mut entries = ivr.entries.clone();
            entries.push(IvrEntry::new());
            ivr.set(Ivr {
                    id: ivr.id,
                    domain_id: ivr.domain_id,
                    name: ivr.name.clone(),
                    exten: ivr.exten.clone(),
                    attrs: ivr.attrs.clone(),
                    entries})
        })
    };

    let form_onsubmit  = {
        let dispatch: Dispatch<Store> = dispatch.clone();
        let store_cloned = store_cloned.clone();
        let loc = loc.clone();
        let nav = nav.clone();
        let ivr = ivr.clone();

        Callback::from(move|event: SubmitEvent| {
            event.prevent_default();

            let ivr = ivr.clone();
            let nav = nav.clone();
            let dispatch: Dispatch<Store> = dispatch.clone();
            let store_cloned = store_cloned.clone();
            let loc = loc.clone();

            let target = event.target().unwrap();
            let form = target.dyn_into().unwrap();
            let form_data = FormData::new_with_form(&form).unwrap();

            let new_attrs = ivr
                .attrs
                .clone()
                .into_iter()
                .map(|a|{
                    let key = a.0;
                    let mut attr = a.1;
                    attr.value = form_data.get(&key).as_string().unwrap();
                    (key, attr)
                })
                .collect::<HashMap<String, IvrAttr>>();

            let new_entries: Vec<IvrEntry> = ivr
                .entries
                .clone()
                .into_iter()
                .enumerate()
                .map(|(i,e)|{
                    let index = i.try_into().unwrap();
                    IvrEntry {
                        id: e.id,
                        ivr_id: ivr.id,
                        digits: form_data.get_all("entry").get(index).as_string().unwrap(),
                        dest_exten: form_data.get_all("destination").get(index).as_string().unwrap()
                    }
                })
                .collect();

            let all_data = Ivr {
                id: ivr.id,
                domain_id: ivr.domain_id,
                name: form_data.get("name").as_string().unwrap(),
                exten: form_data.get("extension").as_string().unwrap(),
                attrs: new_attrs,
                entries: new_entries
            };

            wasm_bindgen_futures::spawn_local(async move {
                let dispatch = dispatch.clone();
                match Service::post(loc.path(), store_cloned.selected_domain_id, all_data).await {
                    Ok(_) => {
                        alert_info("Update IVR successfully.".to_string(), dispatch);
                    }
                    Err(_) => {
                        alert_error("Update IVR failed.".to_string(), dispatch);
                    }
                }
                nav.push(&IvrRoute::Index);
            });
        })
    };
    html! {
        <div class="grow mr-2">
            <Header title= {format!("IVR: {}", ivr.exten.clone())}></Header>
            <div class="divider my-1"></div>
            <form class="w-full" onsubmit={form_onsubmit}>
                <div class="grid grid-cols-3 gap-1">
                    <Label hidden = {props.id != 0}>{"Extension"}</Label>
                    <Input
                        value={ivr.exten.clone()}
                        id="extension"
                        hidden = {props.id != 0}
                    />
                    <Label>{"Name"}</Label>
                    <Input id="name" value={ivr.name.clone()}></Input>
                    {attr_htmls}
                    <Label>{"Entries"}</Label>
                    <div>
                        {entries_html}
                        <div>
                            <div class="btn btn-link btn-sm mr-4" onclick={add_entry}>
                                <Icon icon_id={IconId::LucidePlus}/>
                                {"Add"}
                            </div>
                        </div>
                    </div>
                </div>
                <ActionButtons oncancel={form_oncancel}/>
            </form>
        </div>
    }
}

pub fn ivr_switch(route: IvrRoute) -> Html {
    match route {
        IvrRoute::Index => html!{<IvrList />},
        IvrRoute::Get { id } => html !{<IvrDetails id={id}/>}
    }
}
