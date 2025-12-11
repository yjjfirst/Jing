use yew::prelude::*;
use yewdux::prelude::*;
use crate::models::user::User;
use crate::store::Store;

use crate::components::select::Select;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
    pub value: String
}

#[function_component]
pub fn UserSelect(props: &Props) -> Html {
    let(store, _) = use_store::<Store>();

    let extens: UseStateHandle<Vec<String>> = use_state(||vec![]);

    {
        let extens = extens.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_extension = User::list(store.selected_domain).await;
                extens.set(fetched_extension)
            });
        });
    }
    
    html! {
        <Select
            id={props.id.clone()}
            options = {extens.iter().map(|e|e.to_string()).collect::<Vec<String>>()}
            selected = {props.value.clone()}
        >
        </Select>

    }
}