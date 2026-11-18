use yew::prelude::*;
use super::icon::{CheckedIcon, CrossIcon};

#[derive(Properties, PartialEq)]
pub struct ToggleProps {
    #[prop_or(false)]
    pub checked: bool,
    #[prop_or_default]
    pub onchange: Callback<bool>,
}

#[component]
pub fn Toggle(props: &ToggleProps) -> Html {
    let checked = use_state(|| props.checked);
    let on_change = props.onchange.clone();

    let onchange = {
        let on_change = on_change.clone();
        let checked = checked.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            on_change.emit(input.checked());
            checked.set(input.checked());
        })
    };

    html! {
        <label class="toggle text-base-content">
            <input type="checkbox" checked={*checked} onchange={onchange} />
            <CheckedIcon />
            <CrossIcon />
        </label>
    }
}
