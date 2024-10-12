use web_sys::{EventTarget, FormData, SubmitEvent, HtmlFormElement, HtmlDialogElement};
use wasm_bindgen::JsCast;

use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yew_icons::{Icon, IconId};

use crate::components::input::Input;
use crate::components::file_input::FileInput;
use crate::components::action_buttons::ActionButtons;
use crate::components::header::Header;
use crate::store::{show_alert, Store};
use crate::services::sound_file::SoundFile;
use crate::services::Service;

#[derive(Clone, PartialEq, Properties)] 
pub struct SoundFileProps {
    pub sound: SoundFile,
}

#[derive(Clone, PartialEq, Properties)] 
pub struct SoundFileDetailProps {
    pub id: usize,
}

#[derive(Clone, Routable, PartialEq)]
pub enum SoundFileRoute {
    #[at("/sound-file")]
    Index,
    #[at("/sound-file/:id")]
    Get {id: usize},
}
#[function_component]
pub fn SoundFileListItem(props: &SoundFileProps) -> Html {
    let sound = props.sound.clone();
    let nav = use_navigator().unwrap();

    let onedit: Callback<MouseEvent> = Callback::from(move|_e|{
        nav.push(&SoundFileRoute::Get {id: sound.id});
    });

    html! {
        <tr>
            <th>{props.sound.name.clone()}</th>
            <th>{props.sound.description.clone()}</th>
            <th class="flex justify-end">
                <div class="mr-1">
                    <div onclick={onedit} class="btn btn-square btn-outline btn-sm">
                        <Icon icon_id={IconId::LucideEdit}/>   
                    </div>
                </div>
                <div>
                    <div class="btn btn-square btn-outline btn-sm">
                        <Icon icon_id={IconId::LucideTrash}/>   
                    </div>
                </div>
            </th>             
        </tr>
    }
}

#[function_component]
pub fn SoundFileList() -> Html {
    let (store,_) = use_store::<Store>();
    let sound_files: UseStateHandle<Vec<SoundFile>> = use_state(||vec![]);
    let sound_files_1: UseStateHandle<Vec<SoundFile>> = sound_files.clone();

    let loc = use_location().unwrap().clone();
    let nav = use_navigator().unwrap();
        
    use_effect_with((), move|_| {
        let store = store.clone();
        let sound_files = sound_files_1.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_files: Vec<SoundFile> = Service::index(loc.path(), store.selected_domain.clone()).await;
            sound_files.set(fetched_files);
        });        
    });

    let onadd = Callback::from(move|_e: MouseEvent|{
        nav.push(&SoundFileRoute::Get { id: 0 });
    });

    html! {
        <div class="grow mr-2">
            <Header title="System -> Sound Files"></Header>
            <div class="divider my-1"></div>
            <table class="table table-zebra">
                <thead>
                    <tr>
                        <th>{"File Name"}</th>
                        <th>{"Description"}</th>
                    </tr>
                </thead>
                <tbody>
                {sound_files.iter().map(move|s|{
                    html!{
                        <SoundFileListItem sound={SoundFile {..s.clone()}}/>
                    }
                }).collect::<Html>()}
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
pub fn SoundFileDetail(props: &SoundFileDetailProps) -> Html {
    let nav = use_navigator().unwrap();
    let loc = use_location().unwrap();
    let loc_1 = loc.clone();
    let (store,dispatch) = use_store::<Store>();
    let store_1 = store.clone();
    let id = props.id;

    let sound = use_state(||{
        SoundFile {
            id: 0,
            name: "".to_string(),
            domain_id: 0,
            description: "".to_string(),
        }
    });
    let sound_1 = sound.clone();

    use_effect_with((), move |_| {
        let sound = sound_1.clone();
        let loc = loc.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_out = Service::get(loc.path(), store.selected_domain).await;
            sound.set(fetched_out);
        });
    });

    let form_oncancel = {
        let nav = nav.clone();
        Callback::from(move|_| {
            nav.push(&SoundFileRoute::Index);
        })
    };

    let form_onsubmit = {        
        Callback::from(move|event: SubmitEvent| {
            let target: Option<EventTarget> = event.target();
            let form = target.unwrap().dyn_into::<HtmlFormElement>().unwrap();            
            let form_data = FormData::new_with_form(&form).unwrap();  
            let dispatch = dispatch.clone();
            let loc = loc_1.clone();
            let nav = nav.clone();
            let store = store_1.clone();

            let sound_file = SoundFile {
                id: form_data.get("id").as_string().unwrap().parse::<usize>().unwrap(),
                name: "".to_string(),
                domain_id: 0,
                description: form_data.get("description").as_string().unwrap(),
            };

            wasm_bindgen_futures::spawn_local(async move {
                let dispatch = dispatch.clone();
                let loc = loc.clone();

                match Service::update(loc.path(), store.selected_domain, sound_file)
                    .await {
                        Ok(_) => {
                            show_alert("Update sound file successfully".to_string(), dispatch);
                            nav.push(&SoundFileRoute::Index);            
                        }
                        Err(_) => {
                            show_alert("Update sound file failed".to_string(), dispatch);
                        }
                    }
            });

            event.prevent_default(); 
        })
    };

    html!{
        <div class="grow mr-2">
            <Header title= {format!("Sound File: {}", sound.name)}></Header>
            <div class="divider my-1"></div> 
            <form class="w-full" onsubmit={form_onsubmit} method="POST">
                <Input value={sound.id.to_string()} id="id" hidden=true></Input>
                if id == 0 {
                    <FileInput
                    id="file_name"
                    value=""
                    input_type="file"/>                    
                } else {
                    <Input value={sound.name.clone()} id="name" disabled=true></Input>
                }
                <Input value={sound.description.clone()} id="description"></Input>
                <ActionButtons oncancel={form_oncancel}/>
            </form>
        </div>
    }
}

pub fn sound_file_switch(route: SoundFileRoute) -> Html {
    match route {
        SoundFileRoute::Index => html!{<SoundFileList />},
        SoundFileRoute::Get {id} => html !{<SoundFileDetail id={id}/>}
    }    
}