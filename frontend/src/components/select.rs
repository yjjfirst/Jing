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
            <span 
                for={id.clone()} 
                class="block uppercase tracking-wide text-xs mb-2">
                {props.label.clone()}
            </span>
            <select
                id={id.clone()}
                name={props.name.clone()}
                ref={props.input_ref.clone()}
                class="appearance-none block w-full text-sm rounded-md py-2 px-4 mb-3 mt-2   leading-tight focus:outline outline-1 bg-skin-focus"
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