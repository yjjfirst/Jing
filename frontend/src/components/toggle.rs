use yew::prelude::*;

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
            <svg aria-label="enabled" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                <g
                    stroke-linejoin="round"
                    stroke-linecap="round"
                    stroke-width="4"
                    fill="none"
                    stroke="currentColor"
                >
                    <path d="M20 6 9 17l-5-5"></path>
                </g>
            </svg>
            <svg
                aria-label="disabled"
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="4"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <path d="M18 6 6 18" />
                <path d="m6 6 12 12" />
            </svg>
        </label>
    }
}
