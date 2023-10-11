use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: Option<String>,
    pub label: String,
    pub name: String,
    pub options: Vec<String>,
    pub select: String,
    pub input_ref: NodeRef
}

#[function_component]
pub fn Select(props: &Props) -> Html {
    let id = props
        .id
        .clone()
        .unwrap_or_else(|| "".to_string());
    let options = props.options.clone();
    html!{
        <div>
        <label class="label">
            <span for={id.clone()} 
                class="label-text">
                {props.label.clone()}
            </span>
        </label>        
            <select
                id={id.clone()}
                name={props.name.clone()}
                ref={props.input_ref.clone()}
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
    }
}