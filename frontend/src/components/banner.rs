use yew::prelude::*;
use crate::components::dropdown_menu::DropdownMenu;
use yewdux::prelude::use_store;
use crate::store::{Store, select_domain};
use yew_icons::{Icon, IconId};

#[function_component]
pub fn Banner() -> Html {
    html! {
        <div class="flex justify-end grow items-center ml-4 mr-4">
            <DomainComponent/>
            <div class="btn btn-circle btn-outline btn-sm">
                <Icon icon_id={IconId::LucideUser}/>
            </div>
        </div>
    }
}

#[function_component]
pub fn DomainComponent() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let on_changed = {
        let store = store.clone();
        let dispatch = dispatch.clone();
        Callback::from(move|selected: String|{
            let dispatch = dispatch.clone();
            let mut selected_id = 0;
            for d in store.domains.clone() {
                if selected == d.domain_name {
                    selected_id = d.id
                } 
            };

            select_domain(selected_id, dispatch);
        }
    )};

    let domains = store.domains.clone();
    let selected_id  = store.selected_domain;

    let items: Vec<String> = domains.iter().map(|d| {
        d.domain_name.clone()
    }).collect();

    let mut selected = "".to_string();
    for d in domains {
        if d.id == selected_id {
            selected = d.domain_name.clone();
        }
    }

    html! {
        <div>
            <DropdownMenu {selected} {items} {on_changed}/>
        </div>
    }
}
