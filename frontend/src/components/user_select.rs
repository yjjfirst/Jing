use yew::prelude::*;
use yewdux::prelude::*;
use crate::pages::user::model::User;
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

    let users: UseStateHandle<Vec<String>> = use_state(||vec![]);

    {
        let users = users.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_users = User::list(store.selected_domain).await;
                users.set(fetched_users)
            });
        });
    }
    
    html! {
        <Select
            id={props.id.clone()}
            options = {users.iter().map(|e|e.to_string()).collect::<Vec<String>>()}
            selected = {props.value.clone()}
        >
        </Select>

    }
}