use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AgentRoute {
    #[at("/callcenter/agent")]
    Index,
}

#[function_component]
pub fn AgentList() -> Html {
    html!{

    }
}


pub fn agent_switch(route: AgentRoute) -> Html {
    match route {
        AgentRoute::Index => html!{<AgentList />},
    }
}
