use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum SystemSettingsRoute {
    #[at("/system-settings")]
    Index,
}

#[component]
pub fn SystemSettings() -> Html {
    html! {
        <div>
            <h1>{"System Settings"}</h1>
        </div>
    }
}

pub fn system_settings_switch(route: SystemSettingsRoute) -> Html {
    match route {
        SystemSettingsRoute::Index => html!{
            <SystemSettings />
        }
    }
}

