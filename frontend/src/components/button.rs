use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(PartialEq, Clone)]
pub enum ButtonType {
    Edit,
    User,
    Check,
    X
}

#[derive(Clone, Properties, PartialEq)]
pub struct ButtonProps {
    pub b_type: ButtonType,
    pub children: Option<Children>
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let props = props.clone();

    html! {
        <div class="hover:bg-skin-hover p-2 rounded inline-flex items-center">
            if props.b_type == ButtonType::Edit {
                <Icon icon_id={IconId::LucideEdit}/>
            } else if props.b_type == ButtonType::User {
                <Icon icon_id={IconId::LucideUser}/>
            } else if props.b_type == ButtonType::Check {
                <Icon icon_id={IconId::LucideCheck}/>
            } else if props.b_type == ButtonType::X {
                <Icon icon_id={IconId::LucideX}/>
            }
            if let Some(children) = props.children {
                { for children.iter() }
            }         
        </div>
    }
}


