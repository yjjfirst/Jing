use yew::prelude::*;
use gloo_console::log;

use crate::services::domain::Domain;
use crate::components::dropdown_menu::DropdownMenu;
use crate::components::button::{Button, ButtonType};

#[function_component]
pub fn Banner() -> Html {
    html! {
        <div class="flex justify-end grow items-center bg-skin-fill border-b ml-4">
            <DomainComponent/>
            <Button b_type={ButtonType::User}></Button>
            </div>
    }
}

#[function_component]
pub fn DomainComponent() -> Html {
    let on_changed = {
        Callback::from(move|selected: String|{
            log!(selected);
        }
    )};

    let domains: UseStateHandle<Vec<Domain>> = use_state(||vec![]);
    let dms = domains.clone();

    use_effect_with_deps(move |_| {
        let dms = dms.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_domains = Domain::index().await;
            dms.set(fetched_domains);
        })
    }, ());

    let items: Vec<String> = domains.iter().map(|d| {
        d.domain_name.clone()
    }).collect();

    let selected = domains.first();
    let selected = match selected {
        Some(s) => s.domain_name.clone(),
        None => "Please select domain".to_string()
    };

    html! {
        <DropdownMenu selected={selected} items={items} {on_changed}/>
    }
}
