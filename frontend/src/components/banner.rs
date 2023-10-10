use yew::prelude::*;
use crate::components::dropdown_menu::DropdownMenu;
use crate::components::button::{Button, ButtonIcon, ButtonTheme};
use yewdux::prelude::use_store;
use crate::store::{Store, select_domain};


#[function_component]
pub fn Banner() -> Html {
    html! {
        <div class="flex justify-end grow items-center bg-skin-inverted text-skin-inverted border-b ml-4">
            <DomainComponent/>
            <Button icon={ButtonIcon::User} theme={ButtonTheme::Dark}>{""}</Button>
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
