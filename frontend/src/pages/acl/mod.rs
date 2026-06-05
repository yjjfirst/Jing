
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AclRoute {
    #[at("/acl")]
    Index,
    #[at("/acl/:id")]
    Get {id: usize},
}

pub fn acl_switch(route: AclRoute) -> Html {
    match route {
        AclRoute::Index => html!{<div>{"acl list"}</div>},
        AclRoute::Get { id } => html !{<div>{"acl"}</div>}
    }
}
