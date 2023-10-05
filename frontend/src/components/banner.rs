use std::ops::Deref;
use yew::prelude::*;
use crate::services::domain::Domain;
use crate::components::dropdown_menu::DropdownMenu;
use crate::components::button::{Button, ButtonType};
use yewdux::prelude::use_store;
use crate::store::{Store, select_domain};


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
    let (store, dispatch) = use_store::<Store>();
    let on_changed = {
        let dispatch = dispatch.clone();
        Callback::from(move|selected: String|{
            let dispatch = dispatch.clone();
            select_domain(&selected, dispatch);
        }
    )};

    let domains: UseStateHandle<Vec<Domain>> = use_state(||vec![]);
    let dms = domains.clone();
    let selected_domain : UseStateHandle<String> = use_state(||"".to_string());
    let sel_dm = selected_domain.clone();

    use_effect_with_deps(move |_| {
        let dms = dms.clone();
        let sel_dm = sel_dm.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_domains = Domain::index().await;
            dms.set(fetched_domains.clone());
            sel_dm.set(fetched_domains.first().unwrap().domain_name.clone());
            select_domain(&fetched_domains.first().unwrap().domain_name.clone(), dispatch);
        })
    }, ());

    let items: Vec<String> = domains.iter().map(|d| {
        d.domain_name.clone()
    }).collect();

    let selected = selected_domain.deref().clone();
    html! {
        <div>
            <DropdownMenu {selected} {items} {on_changed}/>
        </div>
    }
}
