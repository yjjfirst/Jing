use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Clone, PartialEq, Properties)] 
pub struct ActionButtonsProps {
    pub oncancel: Callback<MouseEvent>,
}

#[function_component]
pub fn ActionButtons (props: &ActionButtonsProps) -> Html {
    let form_oncancel = props.oncancel.clone();
    html! {
        <div class="flex justify-end mt-4">
            <div>
                <button class="btn btn-success btn-sm mr-4">
                    <Icon icon_id={IconId::LucideCheck}/>
                    {"Apply"}
                </button>
            </div>
            <div>
                <button class="btn btn-warning btn-sm"  onclick={form_oncancel}>
                    <Icon icon_id={IconId::LucideX}/>
                    {"Cancel"}
                </button>
            </div>
        </div>            
    }
}
