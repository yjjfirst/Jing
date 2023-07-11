use yew::prelude::*;
use yew_router::prelude::*;
use crate::main_panel::Route;

#[function_component]
pub fn SideBar() -> Html {
    html! {
        <div class="flex-row bg-zinc-700 text-white ml-1">
            <Logo></Logo>
            <Menu></Menu>
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
pub fn Menu() -> Html {
    html! {
        <div>
            <MenuItem route={Route::Cards}>
                {"Application" }
            </MenuItem>
            <MenuItem route={Route::Cards}>
                {"System"}
            </MenuItem>
        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct MenuItemPros {
    pub children: Children,
    pub route: Route,
}

#[function_component] 
pub fn MenuItem(props: &MenuItemPros) -> Html {
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
