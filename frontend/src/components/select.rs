use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or("".to_string())]
    pub name: String,
    #[prop_or(classes!("w-80"))]
    pub label_width: Classes,
    pub options: Vec<String>,
    #[prop_or("".to_string())]
    pub selected: String,
}

#[function_component]
pub fn Select(props: &Props) -> Html {
    let name = props.name.clone();
    let options = props.options.clone();

    html!{
        <select
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