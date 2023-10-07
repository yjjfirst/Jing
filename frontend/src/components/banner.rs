use yew::prelude::*;
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

    let domains = store.domains.clone();
    let selected_domain  = store.selected_domain.clone();

    let items: Vec<String> = domains.iter().map(|d| {
        d.clone()
    }).collect();

    let selected = selected_domain.clone();
    html! {
        <div>
            <DropdownMenu {selected} {items} {on_changed}/>
        </div>
    }
}
