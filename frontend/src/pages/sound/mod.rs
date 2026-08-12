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
use crate::components::input::Input;
use crate::components::label::Label;
use crate::components::sound_file_select::SoundFileSelect;
use crate::components::action_buttons::ActionButtons;

use crate::store::{alert_info, alert_error, Store};
use crate::models::Service;
use model::{Sound, ApiSound};

#[derive(Clone, Routable, PartialEq)]
pub enum SoundRoute {
    #[at("/sound")]
    Index,
    #[at("/sound/:id")]
    Get {id: usize},
}

#[derive(Clone, PartialEq, Properties)] 
pub struct SoundDetailProps {
    id: usize,
}

#[derive(Clone, PartialEq, Properties)] 
pub struct SoundListItemProps {
    pub sound_id: usize,
    pub name: String,
    pub sound_file: String,
    pub exten: String,
    pub ondel: Callback<usize>
}

#[function_component]
pub fn SoundListItem(props: &SoundListItemProps) -> Html {
    let nav = use_navigator().unwrap();
    let dialog_ref: NodeRef = use_node_ref();
    let dd_ref = dialog_ref.clone(); 
    let loc: Location = use_location().unwrap().clone();
    let (store,_) = use_store::<Store>();
    let ondel = props.ondel.clone();
    let sound_id = props.sound_id;

    let onedit: Callback<MouseEvent> = Callback::from(move |_e|{
        nav.push(&SoundRoute::Get {id: sound_id});
    });

    let onconfirm: Callback<bool> = Callback::from(move|_e: bool|{
        let loc = loc.clone();
        let store = store.clone();
        let ondel = ondel.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let path = format!("{}/{}", loc.path(), sound_id);
            Service::delete(&path, store.clone().selected_domain_id)
                .await
                .unwrap();
            ondel.emit(sound_id);
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
            <th>{props.sound_file.clone()}</th>
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
                contents={format!("Are you sure to delete sound: {}?", props.name.clone())}
                {onconfirm}
                >
            </Dialog>                     

        </tr>
    }
}

#[function_component]
pub fn SoundList() -> Html {
    let loc = use_location().unwrap().clone();
    let nav = use_navigator().unwrap();
    let (store,_) = use_store::<Store>();
    let sounds = use_state(||vec![]);
    {
        let sounds = sounds.clone();
        use_effect_with((), move|_|{
            let sounds = sounds.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_sounds: Vec<ApiSound> = 
                    Service::index(loc.path(), store.selected_domain_id.clone())
                        .await
                        .unwrap();
                sounds.set(fetched_sounds);
            });
        });
    }

    let ondel: Callback<usize> = {
        let sounds = sounds.clone();
        Callback::from(move|id: usize|{
            let filtered: Vec<ApiSound> = sounds
                .iter()
                .filter(|s|s.sound.id != id)
                .map(|s|s.clone())
                .collect();
            sounds.set(filtered);            
        })
    };

    let sounds_html: Vec<Html> = sounds
        .iter()
        .map(|s| {
            html! {
                <SoundListItem 
                    name={s.sound.name.clone()}
                    exten={s.sound.exten.clone()}
                    sound_file={s.sound_file.name.clone()}
                    sound_id={s.sound.id}
                    ondel={ondel.clone()}
                />
            }
        })
        .collect();
    
    let onadd: Callback<MouseEvent> = Callback::from(move|_e: MouseEvent|{
        nav.push(&SoundRoute::Get {id: 0});        
    });

    html! {
        <div class="grow mr-2">
            <Header title="Application -> Sound"></Header>
            <div class="divider my-1"></div>
            <table class="table table-zebra">
                <thead>
                    <tr>
                        <th>{"Extension"}</th>
                        <th>{"Name"}</th>
                        <th>{"Sound File"}</th>
                    </tr>
                    {sounds_html}
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
pub fn SoundDetails(props: &SoundDetailProps) -> Html {
    let id = props.id;
    let loc = use_location().unwrap();
    let nav = use_navigator().unwrap();
    let sound = use_state(||Sound::new());
    let(store, dispatch) = use_store::<Store>();

    {
        let sound = sound.clone();
        let loc = loc.clone();
        let store = store.clone();
        use_effect_with((), move |_|{
            let s = sound.clone();
            let loc = loc.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_sound = 
                    Service::get(loc.path(), store.selected_domain_id)
                        .await
                        .unwrap();
                s.set(fetched_sound);
            });
        });
    }

    let form_oncancel = {
        let nav = nav.clone();
        Callback::from(move|_| {
            nav.push(&SoundRoute::Index);
        })
    };

    let form_onsubmit = {
        let store = store.clone();
        let dispatch = dispatch.clone();
        let sound = sound.clone();
        let nav = nav.clone();
    
        Callback::from(move|e: SubmitEvent| {
            let dispatch = dispatch.clone();
            let loc = loc.clone();
            let store = store.clone();
            let nav = nav.clone();
            
            let form_data = FormData::new_with_form(
                &e
                    .target()
                    .unwrap()
                    .dyn_into::<HtmlFormElement>()
                    .unwrap()).unwrap();
                
            let s = Sound {
                id: sound.id,
                name: form_data.get("name").as_string().unwrap(),
                domain_id: store.selected_domain_id,
                sound_file_id: form_data
                                    .get("sound_file")
                                    .as_string()
                                    .unwrap()
                                    .parse::<usize>()
                                    .unwrap(),
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
                        alert_info("Update sound successfully.".to_string(), dispatch);
                    }
                    Err(_) => {
                        alert_error("Update sound failed.".to_string(), dispatch);
                    }
                }
                nav.push(&SoundRoute::Index);            
            });

            e.prevent_default();    
        })
    };

    html!{
        <div class="grow mr-2">
            <Header title= {format!("Sound: {}", sound.exten.clone())}></Header>
            <div class="divider my-1"></div> 
            <form class="w-full" onsubmit={form_onsubmit}>
            <div class="grid grid-cols-3 gap-1">
                <Label hidden = {id != 0}>{"Extension"}</Label>
                <Input
                    value={sound.exten.clone()}
                    id="exten"
                    hidden = {id != 0}
                />
                <Label>{"Name"}</Label>
                <Input
                    value={sound.name.clone()}
                    id="name"
                />
                <Label>{"Sound File"}</Label>
                <SoundFileSelect id="sound_file" sound_file_id={sound.sound_file_id}/>
            </div>
            <ActionButtons oncancel={form_oncancel}/>
            </form>
        </div>
    }
}

pub fn sound_switch(route: SoundRoute) -> Html {
    match route {
        SoundRoute::Index => html!{<SoundList />},
        SoundRoute::Get { id } => html !{<SoundDetails id={id}/>}
    }
}
