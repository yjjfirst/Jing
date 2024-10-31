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
use crate::components::select::Select;

use crate::services::gateway::Gateway;
use crate::services::route_out::Outbound;
use crate::services::Service;

use yew_icons::{Icon, IconId};

#[derive(Clone, Routable, PartialEq)]
pub enum OutboundRoute {
    #[at("/outbound")]
    Index,
    #[at("/outbound/:id")]
    Get {id: usize},
}
#[derive(Clone, PartialEq, Properties)] 
pub struct OutboundProps {
    pub out: Outbound,
    pub ondel: Callback<usize>    
}
#[derive(Clone, PartialEq, Properties)] 
pub struct OutboundDetailsProps {
    id: usize,
}

#[function_component]
pub fn OutboundListItem(props: &OutboundProps) -> Html {
    let nav = use_navigator().unwrap();
    let out = props.out.clone();
    let dialog_ref: NodeRef = use_node_ref();
    let dd_ref = dialog_ref.clone();    
    let loc: Location = use_location().unwrap().clone();
    let (store,_) = use_store::<Store>();
    let id = props.out.id;
    let ondel = props.ondel.clone();

    let onedit: Callback<MouseEvent> = Callback::from(move|_e|{
        nav.push(&OutboundRoute::Get {id: out.id});
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

    html! {
        <tr>
            <th>{out.id}</th>
            <th>{out.priority}</th>
            <th>{out.condition.clone()}</th>
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
                contents={format!("Are you sure to delete the user: {}?", out.id)}
                {onconfirm}
                >
            </Dialog>                
        </tr>
    }
}

#[function_component] 
pub fn OutboundList() -> Html {
    let loc = use_location().unwrap().clone();
    let (store,_) = use_store::<Store>();
    let nav = use_navigator().unwrap();

    let out_routes: UseStateHandle<Vec<Outbound>> = use_state(||vec![]);
    let out_routes_1 = out_routes.clone();
    let out = out_routes.clone();
    use_effect_with((), move|_|{
        let store = store.clone();
        let out_routes = out_routes.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_routes: Vec<Outbound> = 
                Service::index(loc.path(), store.selected_domain.clone())
                    .await
                    .unwrap();
            out_routes.set(fetched_routes);
        });
    });

    let onadd: Callback<MouseEvent> = Callback::from(move|_e: MouseEvent| {
        nav.push(&OutboundRoute::Get {id: 0});
    });

    let ondel = Callback::from(move| id:usize|{
        let routes = out_routes_1.clone();
        let filtered: Vec<Outbound> = routes
                            .iter()
                            .filter(|r|r.id != id)
                            .map(|r|r.clone())
                            .collect();

        routes.set(filtered);
    });
    
    let out_list: Vec<Html> = out.iter().map(|o|{
        html! {
            <OutboundListItem out={Outbound {..o.clone()}} ondel={ondel.clone()} ></OutboundListItem>
        }
    }).collect();

    html! {
        <div class="grow mr-2">
            <Header title="Connection -> Outbound Routes"></Header>
            <div class="divider my-1"></div>
            <table class="table table-zebra">
                <thead>
                    <tr>
                        <th>{"ID"}</th>
                        <th>{"Priority"}</th>
                        <th>{"Condition"}</th>
                    </tr>
                </thead>
                <tbody>
                    {out_list}
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
pub fn OutboundDetails(_props: &OutboundDetailsProps) -> Html {
    let nav = use_navigator().unwrap();
    let loc = use_location().unwrap();
    let loc_1 = loc.clone();
    let (store,dispatch) = use_store::<Store>();
    let store_1 = store.clone();
    let store_2 = store.clone();

    let out: UseStateHandle<Outbound> = use_state(||Outbound {
        id: 0, condition: "".to_string(), gateway_id:0, priority:100
    });
    let out_1 = out.clone();
    let out_2 = out.clone();

    let gateways: UseStateHandle<Vec<Gateway>> = use_state(||vec![]);
    let gateways_1 = gateways.clone();
    let gateways_2 = gateways.clone();

    use_effect_with((), move |_| {
        let out = out_1.clone();
        let loc = loc.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_out = 
                Service::get(loc.path(), store.selected_domain)
                .await
                .unwrap();
            out.set(fetched_out);
        });
    });

    use_effect_with(out_2.clone(), move |_| {
        let gateways = gateways_1.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_gateway: Vec<Gateway> = 
                    Service::index("/gateway", store_1.selected_domain)
                        .await
                        .unwrap();
                gateways.set(fetched_gateway);
        });
    });

    let form_oncancel = {
        let nav = nav.clone();
        Callback::from(move|_| {
            nav.push(&OutboundRoute::Index);
        })
    };

    let form_onsubmit = {        
        Callback::from(move|event: SubmitEvent| {
            let dispatch = dispatch.clone();
            let loc = loc_1.clone();
            let store = store_2.clone();
            let nav = nav.clone();
            let target: Option<EventTarget> = event.target();
            let form = target.unwrap().dyn_into::<HtmlFormElement>().unwrap();            
            let form_data = FormData::new_with_form(&form).unwrap();  

            let gateway_name = form_data.get("gateway").as_string().unwrap();
            let gateway = Gateway::get_gateway_by_name(gateway_name.clone(), &gateways_2).unwrap();
            let out = Outbound {
                id: form_data.get("id").as_string().unwrap().parse::<usize>().unwrap(),
                condition: form_data.get("condition").as_string().unwrap(),
                priority: form_data.get("priority").as_string().unwrap().parse::<usize>().unwrap(),
                gateway_id: gateway.id
            };

            wasm_bindgen_futures::spawn_local(async move {
                let dispatch = dispatch.clone();
                let loc = loc.clone();

                match Service::post(loc.path(), store.selected_domain, out).await {
                    Ok(_) => {
                        alert_info("Update outbound route successfully.".to_string(), dispatch);
                    }
                    Err(_) => {
                        alert_error("Update outbound route failed.".to_string(), dispatch);
                    }
                }
                nav.push(&OutboundRoute::Index);            
            });

            event.prevent_default(); 
        })
    };

    html!{
        <div class="grow mr-2">
            <Header title= {format!("Outbound: {}", out.id)}></Header>
            <div class="divider my-1"></div> 
            <form class="w-full" onsubmit={form_onsubmit}>
                <Input value={out.id.to_string()} id="id" hidden=true/>
                <Input value={out.priority.to_string()} id="priority"/>
                <Input value={out.condition.clone()} id="condition" />

                if out.gateway_id != 0 && gateways.len() != 0 {
                    <Select 
                        select={Gateway::get_gateway_by_id(out.gateway_id, &gateways).unwrap().gateway_name.clone()}
                        options={gateways
                                    .iter()
                                    .map(|g|{g.gateway_name.clone()})
                                    .collect::<Vec<String>>()}
                        id="gateway"/>
                } else {
                    <Select 
                        select={""}
                            options={gateways
                                    .iter()
                                    .map(|g|{g.gateway_name.clone()})
                                    .collect::<Vec<String>>()}
                        id="gateway"/>                    
                } 
                <ActionButtons oncancel={form_oncancel} />
            </form>
        </div>
    }
}

pub fn outbound_switch(route: OutboundRoute) -> Html {
    match route {
        OutboundRoute::Index => html!{<OutboundList />},
        OutboundRoute::Get { id } => html !{<OutboundDetails id={id}/>}
    }
}