use yew::prelude::*;
use yew_router::prelude::*;
use crate::app::Route;

#[function_component]
pub fn SideBar() -> Html {
    html! {
        <div class="flex-row bg-skin-inverted text-skin-inverted ml-1">
            <Logo></Logo>
            <SidebarMenu/>
        </div>
    }
}

#[function_component]
pub fn Logo() -> Html {
    html! {
        <div class="flex my-4 justify-center">
            <span>{"PBX"}</span>
        </div>
    }
}

#[function_component]
pub fn SidebarMenu() -> Html {
    html! {
        <div>
            <SidebarMenuItem route={Route::Cards}>
                {"Application" }
            </SidebarMenuItem>
            <SidebarMenuItem route={Route::Cards}>
                {"System"}
            </SidebarMenuItem>
        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct SidebarMenuItemPros {
    pub children: Children,
    pub route: Route,
}

#[function_component] 
pub fn SidebarMenuItem(props: &SidebarMenuItemPros) -> Html {
    let nav = use_navigator().unwrap();
    let p = props.clone();
    let onclick = Callback::from(move |_e: MouseEvent| {
        nav.push(&p.route);
    });    

    html! {
        <div {onclick} class="py-2 px-12 hover:bg-skin-inverted_hover text-skin-inverted">
            { for props.children.iter() }
        </div>
    }
}
