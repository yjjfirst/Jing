use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct ParamInputProps {
    pub name: String,
    pub value: String,
    pub range_text: String,
    pub help_text: String,
}
#[component]
pub fn ParamInput(props: &ParamInputProps) -> Html {
    let name = props.name.clone();
    let value = props.value.clone();
    let range_text = props.range_text.clone();
    let help_text = props.help_text.clone();
    let input_ref = use_node_ref();

    let handle_checkbox_change = {
        let input_ref = input_ref.clone();
        Callback::from(move |e: Event| {
            let checkbox: HtmlInputElement = e.target_unchecked_into();
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                if checkbox.checked() == false {
                    input.set_value("true");
                } else {
                    input.set_value("false");
                }
            }
        })
    };

    html! {
        if range_text == "string" || range_text == "domain" {
            <div class="pbx-input-container tooltip tooltip-info" data-tip={help_text}>
            <input class="pbx-input"
                value={value}
                name={name}
            />
            </div>
        } else if range_text == "bool" {
            <div class="pbx-input-container tooltip tooltip-info" data-tip={help_text}>
                <input type="text" class="hidden" name={name} value={value.clone()} ref={input_ref}/>
                <label class="toggle text-base-content">
                    <input type="checkbox" checked={value == "false"} onchange={handle_checkbox_change}/>
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
            </div>
        }
    }
}