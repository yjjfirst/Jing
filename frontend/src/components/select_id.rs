use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or("".to_string())]
    pub id: String,
    #[prop_or(classes!("w-80"))]
    pub label_width: Classes,
    pub options: Vec<String>,
    pub options_id: Vec<usize>,
    #[prop_or(0)]
    pub selected: usize,
}

#[function_component]
pub fn IdSelect(props: &Props) -> Html {
    let id = props.id.clone();
    let mut options = props.options.clone();
    let mut options_id = props.options_id.clone();
    let input_ref = use_node_ref();
    let length = options.len();
    
    options.push("".to_string());
    options_id.push(0);



    let selected = if props.selected != 0 {
        props.selected
    } else {
        match options_id.clone().last() {
            Some(s) => *s,
            None => 0
        }
    };
    
    let handle_change = {
        let input_ref = input_ref.clone();
        Callback::from(move |e: Event| {
            let input_ref = input_ref.clone();
            let input = input_ref
                .cast::<HtmlInputElement>()
                .unwrap();

            let target = e.target_dyn_into::<HtmlSelectElement>().unwrap();
            let selected = target.selected_options().item(0).unwrap();
            input.set_value(&selected.id());
        })
    };
    
    html!{
        <div>
            <select onchange={handle_change}
                class="select select-bordered block w-full col-span-2"
            >
            {
                options.into_iter().enumerate().map(|(i, o)| {
                    html!{
                        if selected == options_id[i] {
                            <option selected=true id={options_id[i].to_string()}>{o}</option>
                        } else if i < length {
                            <option id={options_id[i].to_string()}>{o}</option>
                        } else {
                            <option class="hidden"></option>
                        }
                    }                                       
                }).collect::<Html>()
            }
            </select>
            <input
                ref = {input_ref} 
                name={id.clone()}
                class={"hidden"}
                value={selected.to_string()}/>
        </div>
    }
}