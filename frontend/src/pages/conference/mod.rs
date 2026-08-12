mod model;

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
use crate::components::action_buttons::ActionButtons;

use crate::store::{alert_info, alert_error, Store};
use crate::models::Service;
use model::Conf;

#[derive(Clone, Routable, PartialEq)]
pub enum ConfRoute {
    #[at("/conference")]
    Index,
    #[at("/conference/:id")]
    Get {id: usize},
}

#[derive(Clone, PartialEq, Properties)] 
pub struct ConfDetailProps {
    id: usize,
}


#[derive(Clone, PartialEq, Properties)] 
pub struct ConfListItemProps {
    id: usize,
    exten: String,
    name: String,
    desc: String,
    pub ondel: Callback<usize>
}

#[function_component]
pub fn ConfListItem(props: &ConfListItemProps) -> Html {
    let nav = use_navigator().unwrap();
    let dialog_ref: NodeRef = use_node_ref();
    let loc: Location = use_location().unwrap();
    let (store,_) = use_store::<Store>();
    let ondel = props.ondel.clone();
    let conf_id = props.id;
    
    let onedit: Callback<MouseEvent> = {
        let props = props.clone();
        Callback::from(move |_e|{
            let nav = nav.clone();
            nav.push(&ConfRoute::Get {id: props.id});
        })
    };

    let onconfirm: Callback<bool> = Callback::from(move|_e: bool|{  
        let loc = loc.clone();
        let store = store.clone();
        let ondel = ondel.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let path = format!("{}/{}", loc.path(), conf_id);
            Service::delete(&path, store.clone().selected_domain_id)
                .await
                .unwrap();
            ondel.emit(conf_id);
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
            <th>{props.desc.clone()}</th>
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
                contents={format!("Are you sure to delete Conference: {}?", props.exten.clone())}
                {onconfirm}
                >
            </Dialog>                          
        </tr>
        
    }
    
}

#[function_component]
pub fn ConfList() -> Html {
    let loc = use_location().unwrap().clone();
    let nav = use_navigator().unwrap();    
    let (store,_) = use_store::<Store>();

    let confs = use_state(||vec![]);
    {
        let confs = confs.clone();
        use_effect_with((), move|_|{
            let confs = confs.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_confs: Vec<Conf> = 
                    Service::index(loc.path(), store.selected_domain_id.clone())
                        .await
                        .unwrap();
                confs.set(fetched_confs);
            });
        });
    }
    let ondel: Callback<usize> = {
        let confs = confs.clone();
        Callback::from(move|id: usize|{
            let filtered: Vec<Conf> = confs
                .iter()
                .filter(|c|c.id != id)
                .map(|s|s.clone())
                .collect();
            confs.set(filtered);            
        })
    };    
    let item_list: Vec<Html> = confs.clone().iter().map(|c|{
        html! {
            <ConfListItem 
                id={c.id} 
                exten={c.exten.clone()}
                name={c.name.clone()}                 
                desc={c.description.clone()}
                ondel={ondel.clone()}/>
        }
    }).collect();

    let onadd: Callback<MouseEvent> = Callback::from(move|_e: MouseEvent|{
        nav.push(&ConfRoute::Get {id: 0});        
    });

    html! {
        <div class="grow mr-2">
            <Header title="Application -> Conference"></Header>
            <div class="divider my-1"></div>
            <table class="table table-zebra">
                <thead>
                    <tr>
                        <th>{"Extension"}</th>
                        <th>{"Name"}</th>
                        <th>{"Description"}</th>
                    </tr>
                </thead>
                <tbody>
                    {item_list}
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
pub fn ConfDetails(props: &ConfDetailProps) -> Html{
    let conf_id = props.id;
    let nav = use_navigator().unwrap();
    let loc = use_location().unwrap();
    let conf = use_state(||Conf::new());
    let(store, dispatch) = use_store::<Store>();

    {
        let loc = loc.clone();
        let store = store.clone();
        let conf = conf.clone();
        use_effect_with((), move |_|{
            let conf = conf.clone();
            let loc = loc.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_conf = 
                    Service::get(loc.path(), store.selected_domain_id)
                        .await
                        .unwrap();
                conf.set(fetched_conf);
            });
        });        
    }

    let form_oncancel = {
        let nav = nav.clone();
        Callback::from(move|_| {
            nav.push(&ConfRoute::Index);
        })

    };

    let form_onsubmit = {
        let loc = loc.clone();
        let nav = nav.clone();
        let store = store.clone();
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
            let conf: Conf = Conf {
                id: conf_id,
                name: form_data
                        .get("name")
                        .as_string()
                        .unwrap(),
                exten: form_data  
                        .get("exten")
                        .as_string()
                        .unwrap(),

                description: form_data
                        .get("description")
                        .as_string()
                        .unwrap(),
                conference_profile_id: form_data
                        .get("conference_profile_id")
                        .as_string()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap(),
                domain_id: store.selected_domain_id
            };

            wasm_bindgen_futures::spawn_local(async move {
                let dispatch = dispatch.clone();
                let loc = loc.clone();

                match Service::post(loc.path(), store.selected_domain_id, conf).await {
                    Ok(_) => {
                        alert_info("Update conference successfully.".to_string(), dispatch);
                    }
                    Err(_) => {
                        alert_error("Update conference failed.".to_string(), dispatch);
                    }
                }
                nav.push(&ConfRoute::Index);            
            });

            e.prevent_default();
        })
    };

    html!{
        <div class="grow mr-2">
            <Header title= {format!("Conference: {}", conf.exten.clone())}></Header>
            <div class="divider my-1"></div> 
            <form class="w-full" onsubmit={form_onsubmit}> 
            <div class="grid grid-cols-3 gap-1">
                <Label hidden = {conf_id != 0}>{"Extension"}</Label>
                <Input
                    value={conf.exten.clone()}
                    id="exten"
                    hidden = {conf_id != 0}
                />
                <Label>{"Name"}</Label>
                <Input
                    value={conf.name.clone()}
                    id="name"
                />
                <Label>{"Description"}</Label>
                <Input
                    value={conf.description.clone()}
                    id="description"
                />
                <Label>{"Conference Profile Id"}</Label>
                <Input
                    value={conf.conference_profile_id.to_string()}
                    id="conference_profile_id"
                />
                </div>
                <ActionButtons oncancel={form_oncancel}/>
            </form>
        </div>        
    }
}

pub fn conf_switch(route: ConfRoute) -> Html {
    match route {
        ConfRoute::Index => html!{<ConfList />},
        ConfRoute::Get { id } => html !{<ConfDetails id={id}/>}
    }
}
