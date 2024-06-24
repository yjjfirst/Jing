use yew::prelude::*;
use crate::utils::string::capitalize;
use super::label::Label;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or("".to_string())]
    pub id: String,
    #[prop_or(classes!("w-80"))]
    pub label_width: Classes,
    pub options: Vec<String>,
    pub select: String,
}

#[function_component]
pub fn Select(props: &Props) -> Html {
    let id = props.id.clone();
    let options = props.options.clone();
    let class = props.label_width.clone();
    
    let name = id.clone();
    let label = id.replace("_", " ").replace("-", " ");
    let label = capitalize(&label);
    html!{
        <div class="w-full px-3 mb-6 md:mb-0">
        <div class="flex mb-1">
        <Label class={class}>
            <span for={id.clone()} 
                class="label-text">
                {label}
            </span>
        </Label>        
            <select
                id={id.clone()}
                name={name}
                class="select select-bordered block w-full"
            >
            {
                options.into_iter().map(|o| {
                    html!{
                        if props.select == o {
                            <option selected=true>{o}</option>
                        } else{
                            <option>{o}</option>
                        }
                    }                                       
                }).collect::<Html>()
            }
            </select>

        </div>       
        </div>
    }
}