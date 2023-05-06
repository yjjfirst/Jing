use yew::prelude::*;

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
            <MenuItem>{"Application"}</MenuItem>
            <MenuItem>{"System"}</MenuItem>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct MenuItemPros {
    pub children: Children,
}

#[function_component] 
pub fn MenuItem(props: &MenuItemPros) -> Html {
    html! {
        <div class="py-2 px-12 hover:bg-zinc-500 duration-200 transition-colors">
            { for props.children.iter() }
        </div>
    }
}
