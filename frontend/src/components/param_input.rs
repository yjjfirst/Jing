use yew::prelude::*;
use web_sys::HtmlInputElement;
use regex::Regex;
use crate::components::select_recording::SelectRecording;
use crate::components::select::Select;
use crate::components::icon::{CheckedIcon, CrossIcon};

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
    let enum_re = Regex::new(r"enum\((.*?)\)").unwrap();

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

    let options = {
        if enum_re.is_match(&range_text) {
            let caps = enum_re.captures(&range_text).unwrap();
            caps.get(1)
                .unwrap()
                .as_str()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>()
        } else {
            vec![]
        }
    };

    html! {
        if range_text == "string" || range_text == "domain" {
            <div class="pbx-input-container tooltip tooltip-info" data-tip={help_text}>
                <input class="pbx-input"
                    value={value}
                    name={name}
                />
            </div>
        } else if range_text == "digits" {
            <div class="pbx-input-container tooltip tooltip-info" data-tip={help_text}>
                <input class="pbx-input"
                    value={value}
                    name={name}
                    type="number"
                />
            </div>
        } else if range_text == "bool" {
            <div class="pbx-input-container tooltip tooltip-info" data-tip={help_text}>
                <input type="text" class="hidden" name={name} value={value.clone()} ref={input_ref}/>
                <label class="toggle text-base-content">
                    <input type="checkbox" checked={value == "false"} onchange={handle_checkbox_change}/>
                    <CheckedIcon />
                    <CrossIcon />
                </label>
            </div>
        } else if range_text == "recording" {
            <div class="pbx-input-container tooltip tooltip-info" data-tip={help_text}>
                <SelectRecording
                    name={name}
                    recording_id={value.clone().parse::<usize>().unwrap_or(0)} />
            </div>
        } else if enum_re.is_match(&range_text) {
            <div class="pbx-input-container tooltip tooltip-info" data-tip={help_text}>
                <Select
                    name={name}
                    options={options}
                    selected={value.clone()}
                />
            </div>
        }
    }
}