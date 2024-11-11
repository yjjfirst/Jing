use web_sys::{EventTarget, FormData, SubmitEvent, HtmlFormElement, HtmlDialogElement};
use wasm_bindgen::JsCast;

use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yew_icons::{Icon, IconId};
use crate::store::{alert_info, alert_error, Store};

use crate::components::header::Header;
use crate::components::action_buttons::ActionButtons;
use crate::components::input::Input;
use crate::components::label::Label;
use crate::components::dialog::Dialog;

use crate::services::Service;
use crate::services::gateway::Gateway;

#[derive(Clone, Routable, PartialEq)]
pub enum GatewayRoute {
    #[at("/gateway")]
    Index,
    #[at("/gateway/:id")]
    Get {id: usize},
}

#[derive(Clone, PartialEq, Properties)] 
pub struct GatewayProps {
    pub gateway: Gateway,
    pub ondel: Callback<usize>
}

#[derive(Clone, PartialEq, Properties)] 
pub struct GatewayDetailProps {
    pub id: usize,
}
#[function_component]
pub fn GatewayListItem(props: &GatewayProps) -> Html {
    let gateway = props.gateway.clone();
    let nav = use_navigator().unwrap();
    let loc: Location = use_location().unwrap().clone();
    let (store,_) = use_store::<Store>();    
    let ondel = props.ondel.clone();

    let dialog_ref: NodeRef = use_node_ref();
    let dd_ref = dialog_ref.clone(); 

    let onedit: Callback<MouseEvent> = Callback::from(move |_e|{
        nav.push(&GatewayRoute::Get {id: gateway.id});
    });

    let onconfirm: Callback<bool> = Callback::from(move|_e: bool|{
        let loc = loc.clone();
        let store = store.clone();
        let ondel = ondel.clone();        
        wasm_bindgen_futures::spawn_local(async move {
            let path = format!("{}/{}", loc.path(), gateway.id);
            Service::delete(&path, store.clone().selected_domain)
                .await
                .unwrap();
            ondel.emit(gateway.id);
        })        
    });
    
    let ondel: Callback<MouseEvent> = Callback::from(move |_e| {
        let d = dd_ref.cast::<HtmlDialogElement>().unwrap();
        d.show_modal().unwrap();
    });

    html! {
        <tr>
            <th>{gateway.gateway_name.clone()}</th>
            <th>{gateway.proxy.clone()}</th>
            <th>{gateway.register.clone()}</th>
            <th>{gateway.username.clone()}</th>
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
                contents={format!("Are you sure to delete gateway: {}?", gateway.gateway_name.clone())}
                {onconfirm}
                >
            </Dialog>                     
        </tr>
    }
}

#[function_component]
pub fn GatewayList() -> Html {
    let loc = use_location().unwrap().clone();
    let nav = use_navigator().unwrap();

    let (store,_) = use_store::<Store>();
    let gateways: UseStateHandle<Vec<Gateway>> = use_state(||vec![]);
    let gateways_1 = gateways.clone();
    let gateways_2 = gateways.clone();

    use_effect_with((), move|_|{
        let gateways = gateways_1.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_gateways: Vec<Gateway> = 
                Service::index(loc.path(), store.selected_domain.clone())
                    .await
                    .unwrap();
            gateways.set(fetched_gateways);
        });
    });

    let ondel: Callback<usize> = Callback::from(move|id: usize|{
        let gateways = gateways_2.clone();
        let filtered: Vec<Gateway> = gateways
            .iter()
            .filter(|g| {g.id !=id})
            .map(|g|{g.clone()})
            .collect();

        gateways.set(filtered);
    });

    let onadd: Callback<MouseEvent> = Callback::from(move|_e: MouseEvent|{
        nav.push(&GatewayRoute::Get {id: 0});        
    });

    let gateways_list: Vec<Html> = gateways.iter().map(|g|{
        html! {
            <GatewayListItem gateway={Gateway {..g.clone()}} ondel={ondel.clone()}></GatewayListItem>
        }
    }).collect();

    html! {
        <div class="grow mr-2">
            <Header title="Connection -> Gateway"></Header>
            <div class="divider my-1"></div>
            <table class="table table-zebra">
                <thead>
                    <tr>
                        <th>{"Name"}</th>
                        <th>{"Proxy"}</th>
                        <th>{"Register"}</th>
                        <th>{"Username"}</th>
                    </tr>
                </thead>
                <tbody>
                {gateways_list}
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
pub fn GatewayDetails(props: &GatewayDetailProps) -> Html {
    let nav = use_navigator().unwrap();
    let loc = use_location().unwrap();
    let loc_1 = loc.clone();

    let id = props.id;
    let gateway = use_state(||Gateway::new());
    let g = gateway.clone();
    let(store, dispatch) = use_store::<Store>();
    let store_cloned = store.clone();
    use_effect_with((), move |_|{
        let gateway = g.clone();
        let loc = loc_1.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_gateway = 
                Service::get(loc.path(), store.selected_domain)
                    .await
                    .unwrap();
            gateway.set(fetched_gateway);
        });
    });

    let form_oncancel = {
        let nav = nav.clone();
        Callback::from(move|_| {
            nav.push(&GatewayRoute::Index);
        })

    };
    let form_onsubmit = {
        let dispatch = dispatch.clone();
        Callback::from(move|e: SubmitEvent| {
            let store = store_cloned.clone();
            let target: Option<EventTarget> = e.target();
            let form = target.unwrap().dyn_into::<HtmlFormElement>().unwrap();            
            let form_data = FormData::new_with_form(&form).unwrap();
            let dispatch = dispatch.clone();
            let loc = loc.clone();
            let nav = nav.clone();

            let gateway = Gateway {
                id,
                gateway_name: form_data.get("name").as_string().unwrap(),
                proxy: form_data.get("proxy").as_string().unwrap(),
                register: form_data.get("register").as_string().unwrap(),
                username: form_data.get("username").as_string().unwrap(),
                password: form_data.get("password").as_string().unwrap(),
                profile_id: form_data.get("profile_id")
                    .as_string()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
            };

            wasm_bindgen_futures::spawn_local(async move {
                let dispatch = dispatch.clone();
                let loc = loc.clone();

                match Service::post(loc.path(), store.selected_domain, gateway).await {
                    Ok(_) => {
                        alert_info("Update gateway successfully.".to_string(), dispatch);
                    }
                    Err(_) => {
                        alert_error("Update gateway failed.".to_string(), dispatch);
                    }
                }
                nav.push(&GatewayRoute::Index);            
            });
                        
            e.prevent_default();            
        })
    };

    html! {
        <div class="grow mr-2">
            <Header title= {format!("Gateway: {}", gateway.gateway_name.clone())}></Header>
            <div class="divider my-1"></div> 
            <form class="w-full" onsubmit={form_onsubmit}>
            <div class="grid grid-cols-3 gap-1">
                <Label>{"name"}</Label>
                <Input
                    value={gateway.gateway_name.clone()}
                    id="name"
                />
                <Label>{"proxy"}</Label>
                <Input
                    value={gateway.proxy.clone()}
                    id="proxy"
                />
                <Label>{"register"}</Label>
                <Input
                    value={gateway.register.clone()}
                    id="register"
                />
                <Label>{"Username"}</Label>
                <Input
                    value={gateway.username.clone()}
                    id="username"
                />
                <Label>{"Password"}</Label>
                <Input
                    value={gateway.password.clone()}
                    id="password"
                />
                <Input hidden=true
                    value={gateway.profile_id.to_string()}
                    id="profile_id"
                />                
            </div>
            <ActionButtons oncancel={form_oncancel}/>
            </form>
        </div>
    }
}
pub fn gateway_switch(route: GatewayRoute) -> Html {
    match route {
        GatewayRoute::Index => html!{<GatewayList />},
        GatewayRoute::Get { id } => html !{<GatewayDetails id={id}/>}
    }
}