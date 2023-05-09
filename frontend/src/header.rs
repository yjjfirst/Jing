use yew::prelude::*;
use yew_router::prelude::*;
use crate::button::{Button, ButtonType};

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub title: String
}

#[function_component]
pub fn Header(props: &HeaderProps) -> Html {
    let title = props.title.clone();
    html! {
        <div class="flex justify-between grow items-center border-b-zinc-400 border-b h-16">
            <h1>
                {title}
            </h1>
            <Button b_type={ButtonType::User}></Button>
        </div>    
    }
}
