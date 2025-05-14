
pub mod queues;
pub mod agents;
pub mod tiers;

use queues::{QueueRoute, queue_switch};
use agents::{AgentRoute, agent_switch};
use tiers::{TierRoute,tier_switch};

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;
use yew_icons::{Icon, IconId};

use crate::components::header::Header;

#[derive(Clone, Routable, PartialEq)]
pub enum CallcenterRootRoute {
    #[at("/callcenter/queue")]
    Queue,
    #[at("/callcenter/agent")]
    Agent,
    #[at("/callcenter/tier")]
    Tier,
}

#[derive(Clone, PartialEq, Properties)] 
pub struct CallcenterDetailProps {
    pub id: usize,
}

#[function_component]
pub fn Callcenter() -> Html {
    let nav = use_navigator().unwrap();    
    let loc = use_location().unwrap().clone();

    let handle_change = Callback::from( move |e: Event| { 
        let i = e
            .target_dyn_into::<HtmlInputElement>()
            .unwrap();
        if i.value() == "queue" {
            nav.push(&CallcenterRootRoute::Queue);
        } else if i.value() == "agent" {
            nav.push(&CallcenterRootRoute::Agent);
        } else {
            nav.push(&CallcenterRootRoute::Tier);
        }
    });
    html!{
        <div class="grow mr-2">
            <Header title="Application -> Call Center"></Header>
            <div class="divider my-1"></div>
            <div class="tabs tabs-border">
                <input type="radio" 
                    name="callcenter_tab" 
                    class="tab" 
                    aria-label={"Queues"}
                    value="queue"
                    onchange={handle_change.clone()}
                    checked={loc.path().contains("queue")}/>
                <div class="tab-content">
                    <Switch<QueueRoute> render={queue_switch} />
                </div>

                <input type="radio" 
                    name="callcenter_tab" 
                    class="tab" 
                    value="agent"
                    onchange={handle_change.clone()}
                    aria-label={"Agents"}  
                    checked={loc.path().contains("agent")}/>
                <div class="tab-content">
                    <Switch<AgentRoute> render={agent_switch} />
                </div>

                <input type="radio" 
                    name="callcenter_tab" 
                    class="tab"
                    value="tier"
                    onchange={handle_change.clone()}
                    aria-label={"Tiers"} 
                    checked={loc.path().contains("tier")}/>
                <div class="tab-content">
                    <Switch<TierRoute> render={tier_switch} />
                </div>                    
            </div>
        </div>            
    }
}

pub fn callcenter_root_switch(route: CallcenterRootRoute) -> Html {
    match route {
        CallcenterRootRoute::Queue => html!{<Callcenter/>},
        CallcenterRootRoute::Agent => html!{<Callcenter/>},
        CallcenterRootRoute::Tier => html!{<Callcenter/>},
    }
}
