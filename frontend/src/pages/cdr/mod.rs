mod model;

use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use model::Cdr;
use crate::components::header::Header;
use crate::store::Store;
use crate::models::Service;

#[derive(Clone, Routable, PartialEq)]
pub enum CdrRoute {
    #[at("/cdr")]
    Index,
}

#[function_component]
pub fn CdrList() -> Html {
    let loc = use_location().unwrap().clone();
    let (store,_) = use_store::<Store>();
    let cdrs: UseStateHandle<Vec<Cdr>> = use_state(||vec![]);
    let cdrs_1 = cdrs.clone();

    use_effect_with((), move|_|{
        let store = store.clone();
        let cdrs = cdrs_1.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_routes: Vec<Cdr> = 
                Service::index(loc.path(), store.selected_domain_id.clone())
                    .await
                    .unwrap();
            cdrs.set(fetched_routes);
        });
    });    
    html!{
        <div class="grow mr-2">
            <Header title="Report -> CDR"></Header>
            <div class="divider my-1"></div>
            <table class="table table-zebra">
                <thead>
                    <tr>
                        <th>{"Caller ID Name"}</th>
                        <th>{"Caller ID Number"}</th>
                        <th>{"Callee Number"}</th>
                        <th>{"Start At"}</th>
                        <th>{"End At"}</th>
                        <th>{"Duration"}</th>
                        <th>{"Billsec"}</th>
                        <th>{"Hangup Cause"}</th>
                    </tr>
                </thead>            
                <tbody>
                {   
                    cdrs.iter().map(|c|{
                       html! {
                           <tr>
                               <td>{c.caller_id_name.clone()}</td>
                               <td>{c.caller_id_number.clone()}</td>
                               <td>{c.destination_number.clone()}</td>
                               <td>{c.start_stamp.clone().format("%Y-%m-%d %H:%M:%S").to_string()}</td>
                               <td>{c.end_stamp.clone().format("%Y-%m-%d %H:%M:%S").to_string()}</td>
                               <td>{c.duration}</td>
                               <td>{c.billsec}</td>
                               <td>{c.hangup_cause.clone()}</td>                                
                               </tr>
                           }
                   }).collect::<Html>() 
                }

                </tbody>
            </table>
        </div>        
    }    
}

pub fn cdr_switch(route: CdrRoute) -> Html {
    match route {
        CdrRoute::Index => html!{<CdrList></CdrList>}
    }
}