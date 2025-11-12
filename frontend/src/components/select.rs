use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or("".to_string())]
    pub id: String,
    #[prop_or(classes!("w-80"))]
    pub label_width: Classes,
    pub options: Vec<String>,
    #[prop_or("".to_string())]
    pub selected: String,
}

#[function_component]
pub fn Select(props: &Props) -> Html {
    let id = props.id.clone();
    let options = props.options.clone();
    let name = id.clone();

    html!{
        <select
            id={id.clone()}
            name={name}
            class="select select-bordered block w-full col-span-2"
        >
        {
            options.into_iter().map(|o| {
                html!{
                    if props.selected == o {
                        <option selected=true>{o}</option>
                    } else{
                        <option>{o}</option>
                    }
                }                                       
            }).collect::<Html>()
        }
        </select>
    }
}