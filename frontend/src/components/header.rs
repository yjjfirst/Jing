use std::ops::Deref;

use yew::prelude::*;
use yew_icons::{Icon, IconId};
use crate::components::button::{Button, ButtonType};
use gloo_console::log;
use web_sys::HtmlAnchorElement;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct DropdownMenuProps {
    pub selected: String,
    pub items: Vec<String>,
    pub on_changed: Callback<String>
}

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub title: String,
}

#[function_component]
pub fn DropdownMenu(props: &DropdownMenuProps) -> Html {
    let items = props.items.clone();
    let selected = use_state(||props.selected.clone());
    let dropped = use_state(||false);
    let onclick = {
        let dropped = dropped.clone();
        Callback::from(move|_| dropped.set(!(*dropped)))
    };

    let mut icon_class = vec!["ml-1", "transform scale-50"];
    if *dropped {
        icon_class.push("transfrom rotate-180");
    }

    let mut menu_class = vec!["text-skin-inverted bg-skin-inverted rounded absolute left-0 top-full transition-all"];
    if !*dropped {
        menu_class.push("invisible");
    }

    let menu_onclick = {
        let cb = props.on_changed.clone();
        let dropped: UseStateHandle<bool> = dropped.clone();
        let selected = selected.clone();
        Callback::from(move|e: MouseEvent|{
            e.prevent_default();
            let a = e.target().unwrap().dyn_into::<HtmlAnchorElement>().unwrap();
            selected.set(a.rel());
            cb.emit(a.rel().to_string());
            dropped.set(false)
        })
    };

    let onfocusout: Callback<FocusEvent>= {
        let dropped: UseStateHandle<bool> = dropped.clone();
        Callback::from(move|_|dropped.set(false))
    };

    html! {
        <div class="flex items-center">
            <div class="py-2 group relative">
                <button {onclick} {onfocusout} class="rounded inline-flex py-2">{selected.deref()}
                    <Icon icon_id={IconId::LucideChevronDown} class={classes!(icon_class)}/>
                </button>
                <nav class={classes!(menu_class)}>
                    <ul class="py-1">
                        {items.iter().map(|i|html!{
                            <li>
                                <a onclick={menu_onclick.clone()} rel={i.to_string()} class="block px-4 py-2 hover:bg-skin-inverted_hover">
                                    {i.clone()}
                                </a>
                            </li>
                        }).collect::<Vec<Html>>()}
                    </ul>
                </nav>
            </div>
        </div>
    }

}

#[function_component]
pub fn Domain() -> Html {
    let items = vec![
        "ttt".to_string(),
        "teleman.me".to_string(),
        "192.168.100.1".to_string(),
        "ttt".to_string(),
        "teleman.me".to_string(),
        "192.168.100.1".to_string()];

    let on_changed = {
        Callback::from(move|selected: String|{
            log!(selected);
        }
    )};

    html! {
        <DropdownMenu selected={"ttt"} items={items} {on_changed}/>
    }
}

#[function_component]
pub fn Header(props: &HeaderProps) -> Html {
    let title = props.title.clone();
    html! {
        <div class="flex justify-between grow items-center bg-skin-fill border-b h-24 mb-4">
            <div>
                <Domain/>
                <h1>
                    {title}
                </h1>
            </div>
            <Button b_type={ButtonType::User}></Button>
        </div>
    }
}
