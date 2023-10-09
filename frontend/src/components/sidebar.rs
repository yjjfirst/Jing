use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlAnchorElement;
use crate::app::Route;
use wasm_bindgen::JsCast;


#[derive(Clone, Properties, PartialEq)]
pub struct SidebarMenuItemPros {
    pub caption: String,
    pub route: Route,
    pub onclick: Callback<String>,
    pub selected: bool
}

#[function_component]
pub fn SideBar() -> Html {
    html! {
        <div class="w-48 flex-row bg-skin-inverted text-skin-inverted">
            <SidebarMenu/>
        </div>
    }
}

#[function_component]
pub fn SidebarMenu() -> Html {  
    let selected = use_state(||"".to_string());  
    let onclick = {
        let selected = selected.clone();
        Callback::from(move |caption: String|{
            selected.set(caption);
        })
    };

    let items = vec![
        SidebarMenuItemPros {
            caption: "Application".to_string(),
            route: Route::Cards,
            onclick: onclick.clone(),
            selected: false
        },
        SidebarMenuItemPros {
            caption: "System".to_string(),
            route: Route::Cards,
            onclick: onclick.clone(),
            selected: false
        }
    ];

    html! {
        <div class="flex flex-col">
            {items.iter().map(|i|{
                html! {
                    <SidebarMenuItem 
                        route={i.route.clone()} 
                        caption={i.caption.clone()} 
                        onclick={i.onclick.clone()}
                        selected = {*selected == i.caption.clone()}>
                    </SidebarMenuItem>
                }
            }).collect::<Vec<Html>>()}
        </div>
    }
}

#[function_component] 
pub fn SidebarMenuItem(props: &SidebarMenuItemPros) -> Html {
    let nav = use_navigator().unwrap();
    let props_onclick = props.onclick.clone();
    let p = props.clone();

    let onclick = Callback::from(move |e: MouseEvent| {
        let a: HtmlAnchorElement = e.target().unwrap().dyn_into::<HtmlAnchorElement>().unwrap();        
        props_onclick.emit(a.rel().to_string());
        nav.push(&p.route);
    }); 

    let mut classes = vec!["block py-2 px-12 hover:bg-skin-inverted_hover text-skin-inverted"];
    if  props.selected {
        classes.push("border-l-4 border-orange-500");
    }

    html! {
        <a {onclick} class={classes!(classes)} rel={props.caption.clone()}>
            { props.caption.clone() }
        </a>
    }
}
