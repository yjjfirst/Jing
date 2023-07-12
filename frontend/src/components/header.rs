use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::components::button::{Button, ButtonType};

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub title: String,
}

#[function_component]
pub fn Domain() -> Html {
    html! {
        <div class="flex items-center">
            <div class="py-2 group relative">
                <button class="rounded inline-flex py-2">{"45.76.77.24"}
                    <Icon icon_id={IconId::LucideChevronDown} class="ml-1 transform scale-50 group-focus-within:transform group-focus-within:rotate-180"/>
                </button>
                <nav class="text-skin-inverted bg-skin-inverted invisible rounded absolute left-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1">
                    <ul class="py-1">
                        <li>
                            <a href="#" class="block px-4 py-2 hover:bg-skin-inverted_hover">
                                {"45.76.77.24"}
                            </a>
                        </li>
                        <li>
                            <a href="#" class="block px-4 py-2 hover:bg-skin-inverted_hover">
                                {"teleman.me"}
                            </a>
                        </li>
                    </ul>
                </nav>
            </div>
        </div>
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
