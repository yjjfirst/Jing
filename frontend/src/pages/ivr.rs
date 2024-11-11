use web_sys::{FormData, SubmitEvent, HtmlFormElement, HtmlDialogElement};
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

use crate::store::{alert_info, alert_error, Store};
use crate::services::Service;
use crate::services::ivr::{IvrAllData, Ivr};

#[derive(Clone, Routable, PartialEq)]
pub enum IvrRoute {
    #[at("/ivr")]
    Index,
    #[at("/ivr/:id")]
    Get {id: usize},
}

#[derive(Clone, PartialEq, Properties)] 
pub struct IvrDetailProps {
    id: usize,
}

#[derive(Clone, PartialEq, Properties)] 
pub struct IvrListItemProps {
    id: usize,
    exten: String,
    name: String,
    pub ondel: Callback<usize>
}

#[derive(Clone, PartialEq, Properties)] 
pub struct IvrEntryProps {
    pub digits: String,
    pub dest: String
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
            Service::delete(&path, store.clone().selected_domain)
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
                    Service::index(loc.path(), store.selected_domain.clone())
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
        Callback::from(move|id: usize|{
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
pub fn IvrDetails(props: &IvrDetailProps) -> Html {
    let nav = use_navigator().unwrap();
    let ivr = use_state(||IvrAllData::new());
    let loc = use_location().unwrap();
    let(store, dispatch) = use_store::<Store>();

    let form_oncancel = {
        let nav = nav.clone();
        Callback::from(move|_| {
            nav.push(&IvrRoute::Index);
        })
    };
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
                    Service::get(loc.path(), store.selected_domain)
                        .await
                        .unwrap();
                    ivr.set(fetched_ivr);
                
            });
        });
    }

    let mut items: Vec<Html> = vec![];

    for i in ivr.attrs.iter() {
        items.push(html!{
            <Label>{i.name.clone()}</Label>
        });
        items.push(html!{
            <Input id={i.name.clone()} value={i.value.clone()} />
        });

    }

    html! {
        <div class="grow mr-2">
            <Header title= {format!("Conference: {}", ivr.ivr.exten.clone())}></Header>
            <div class="divider my-1"></div> 
            <form class="w-full"> 
                <div class="grid grid-cols-3 gap-1">
                {items}
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
