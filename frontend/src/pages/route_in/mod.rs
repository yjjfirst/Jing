pub mod model;

use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;
use yewdux::prelude::*;
use web_sys::{EventTarget, FormData, SubmitEvent, HtmlFormElement, HtmlDialogElement};
use wasm_bindgen::JsCast;

use crate::store::{alert_info, alert_error, Store};
use crate::components::header::Header;
use crate::components::dialog::Dialog;
use crate::components::action_buttons::ActionButtons;
use crate::components::input::Input;
use crate::components::label::Label;
use crate::components::exten_select::ExtenionSelect;
use model::Inbound;
use crate::models::Service;

use yew_icons::{Icon, IconId};

#[derive(Clone, Routable, PartialEq)]
pub enum InboundRoute {
    #[at("/inbound")]
    Index,
    #[at("/inbound/:id")]
    Get {id: usize},
}
#[derive(Clone, PartialEq, Properties)] 
pub struct InboundProps {
    pub inbound: Inbound,
    pub ondel: Callback<usize>    
}
#[derive(Clone, PartialEq, Properties)] 
pub struct InboundDetailsProps {
    id: usize,
}

#[function_component]
pub fn InboundListItem(props: &InboundProps) -> Html {
    let nav = use_navigator().unwrap();
    let loc: Location = use_location().unwrap().clone();
    let (store,_) = use_store::<Store>();
    let id = props.inbound.id;
    let ondel = props.ondel.clone();
    
    let inbound = props.inbound.clone();
    let dialog_ref: NodeRef = use_node_ref();
    let dd_ref = dialog_ref.clone(); 

    let onedit: Callback<MouseEvent> = Callback::from(move|_e|{
        nav.push(&InboundRoute::Get {id: inbound.id});
    });

    let onconfirm: Callback<bool> = Callback::from(move|_e: bool|{
        let loc = loc.clone();
        let store = store.clone();
        let ondel = ondel.clone();        

        wasm_bindgen_futures::spawn_local(async move {
            let path = format!("{}/{}", loc.path(), id);
            Service::delete(&path, store.clone().selected_domain)
                .await
                .unwrap();
            ondel.emit(id);
        })
    });  

    let ondel = Callback::from(move|_e: MouseEvent|{
        let d = dd_ref.cast::<HtmlDialogElement>().unwrap();
        d.show_modal().unwrap();  
    });

    html!{
        <tr>
            <th>{inbound.id}</th>
            <th>{inbound.context}</th>
            <th>{inbound.condition.clone()}</th>
            <th>{inbound.dest_extension.clone()}</th>

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
                contents={format!("Are you sure to delete the user: {}?", inbound.id)}
                {onconfirm}
                >
            </Dialog>                
        </tr>
    }
}

#[function_component] 
pub fn InboundList() -> Html {
    let nav = use_navigator().unwrap();
    let loc = use_location().unwrap().clone();
    let (store,_) = use_store::<Store>();

    let in_routes: UseStateHandle<Vec<Inbound>> = use_state(||vec![]);
    let in_routes_1 = in_routes.clone();
    let in_routes_2 = in_routes.clone();

    use_effect_with((), move|_|{
        let store = store.clone();
        let in_routes = in_routes_1.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_routes: Vec<Inbound> = 
                Service::index(loc.path(), store.selected_domain.clone())
                    .await
                    .unwrap();
            in_routes.set(fetched_routes);
        });
    });

    let ondel = Callback::from(move| id: usize|{
        let routes = in_routes_2.clone();
        let filtered: Vec<Inbound> = routes
                            .iter()
                            .filter(|r|r.id != id)
                            .map(|r|r.clone())
                            .collect();

        routes.set(filtered);
    });

    let onadd = Callback::from(move|_e: MouseEvent|{
        nav.push(&InboundRoute::Get { id: 0 });
    });

    let in_list: Vec<Html> = in_routes.iter().map(|i| {
        html!{
            <InboundListItem inbound={Inbound {..i.clone()}} ondel={ondel.clone()}></InboundListItem>
        }
    }).collect();
    html! {
        <div class="grow mr-2">
            <Header title="Connection -> Inbound Routes"></Header>
            <div class="divider my-1"></div>
            <table class="table table-zebra">
                <thead>
                    <tr>
                        <th>{"ID"}</th>
                        <th>{"Context"}</th>
                        <th>{"Condition"}</th>
                        <th>{"Destination"}</th>
                    </tr>
                </thead>
                <tbody>
                {in_list}
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
pub fn InboundDetails(_props: &InboundDetailsProps) -> Html {
    let loc = use_location().unwrap();
    let loc_1 = loc.clone();
    let nav = use_navigator().unwrap();

    let (store,dispatch) = use_store::<Store>();
    let store_1 = store.clone();

    let inbound: UseStateHandle<Inbound> = use_state(|| Inbound {
        id: 0, 
        condition: "".to_string(), 
        context: "".to_string(), 
        dest_extension: "".to_string()
    });
    let inbound_1 = inbound.clone();

    use_effect_with((), move |_| {
        let inbound = inbound_1.clone();
        let loc = loc.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_out = 
                Service::get(loc.path(), store.selected_domain)
                    .await
                    .unwrap();
            inbound.set(fetched_out);
        });
    });
    
    let form_oncancel = {
        let nav = nav.clone();
        Callback::from(move|_| {
            nav.push(&InboundRoute::Index);
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

            let inbound = Inbound {
                id: form_data.get("id").as_string().unwrap().parse::<usize>().unwrap(),
                context: form_data.get("context").as_string().unwrap(),
                condition: form_data.get("condition").as_string().unwrap(),
                dest_extension: form_data.get("destination").as_string().unwrap()
            };

            wasm_bindgen_futures::spawn_local(async move {
                let dispatch = dispatch.clone();
                let loc = loc.clone();

                match Service::post(loc.path(), store.selected_domain, inbound).await {
                    Ok(_) => {
                        alert_info("Update inbound route successfully.".to_string(), dispatch);
                    }
                    Err(_) => {
                        alert_error("Update inbound route failed.".to_string(), dispatch);
                    }
                }
                nav.push(&InboundRoute::Index);            
            });

            event.prevent_default(); 
        })
    };

    html!{
        <div class="grow mr-2">
            <Header title= {format!("Inbound: {}", inbound.id)}></Header>
            <div class="divider my-1"></div> 
            <form class="w-full" onsubmit={form_onsubmit}>
            <div class="grid grid-cols-3 gap-1">
                <Input value={inbound.id.to_string()} id="id" hidden=true></Input>
                <Label>{"Condition"}</Label>
                <Input value={inbound.condition.clone()} id="condition"></Input>
                <Label>{"Context"}</Label>
                <Input value={inbound.context.clone()} id="context"></Input>
                <Label>{"Destination"}</Label>
                <ExtenionSelect id="destination" value={inbound.dest_extension.clone()}/>
                </div>        
                <ActionButtons oncancel={form_oncancel} />
            </form>
        </div>
    }
}

pub fn inbound_switch(route: InboundRoute) -> Html {
    match route {
        InboundRoute::Index => html!{<InboundList />},
        InboundRoute::Get { id } => html !{<InboundDetails id={id}/>}
    }
}