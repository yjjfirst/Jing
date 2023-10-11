use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub title: String,
}

#[function_component]
pub fn Header(props: &HeaderProps) -> Html {
    let title = props.title.clone();
    html! {
        <div class="flex justify-between grow items-center my-4 mr-2">
            <div>
                <h1>
                    {title}
                </h1>
            </div>
        </div>
    }
}
