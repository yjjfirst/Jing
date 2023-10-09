use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(PartialEq, Clone)]
pub enum ButtonIcon {
    Edit,
    User,
    Check,
    X
}

#[derive(PartialEq, Clone)]
pub enum ButtonTheme {
    Dark,
    Light
}

#[derive(Clone, Properties, PartialEq)]
pub struct ButtonProps {
    pub icon: ButtonIcon,
    pub theme: ButtonTheme,
    pub children: Option<Children>
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let props = props.clone();
    let theme = props.theme;
    let icon = props.icon;

    let class = if theme == ButtonTheme::Dark {
        "hover:bg-skin-inverted_hover"
    } else {
        "hover:bg-skin-hover"
    };

    html! {
        <div class={classes!(class, "p-2", "rounded", "inline-flex", "items-center")}>
            if icon == ButtonIcon::Edit {
                <Icon icon_id={IconId::LucideEdit}/>
            } else if icon == ButtonIcon::User {
                <Icon icon_id={IconId::LucideUser}/>
            } else if icon == ButtonIcon::Check {
                <Icon icon_id={IconId::LucideCheck}/>
            } else if icon == ButtonIcon::X {
                <Icon icon_id={IconId::LucideX}/>
            }
            if let Some(children) = props.children {
                { for children.iter() }
            }         
        </div>
    }
}


