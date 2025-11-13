use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum TierRoute {
    #[at("/callcenter/tier")]
    Index,
}

#[function_component]
pub fn Tiers() -> Html {
    html!{
        <div>{"Tiers"}</div>
    }
}

pub fn tier_switch(route: TierRoute) -> Html {
    match route {
        TierRoute::Index => html!{<Tiers />},
    }
}
