use yew::prelude::*;
use yew_router::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(PartialEq, Clone)]
pub enum ButtonType {
    Edit,
    User,
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub b_type: ButtonType,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let b_type = props.b_type.clone();

    html! {
        <div class="hover:bg-skin-hover p-2 rounded inline-flex items-center">
            if b_type == ButtonType::Edit {
                <Icon icon_id={IconId::LucideEdit}/>
            } else if b_type == ButtonType::User {
                <Icon icon_id={IconId::LucideUser}/>
            }
        </div>
    }
}


