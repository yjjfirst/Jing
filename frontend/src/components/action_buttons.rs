use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Clone, PartialEq, Properties)] 
pub struct ActionButtonsProps {
    #[prop_or_default]
    pub oncancel: Callback<MouseEvent>,
    #[prop_or(true)]
    pub has_cancel: bool,
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
            if props.has_cancel {
                <div>
                    <button class="btn btn-warning btn-sm"  onclick={form_oncancel}>
                        <Icon icon_id={IconId::LucideX}/>
                        {"Cancel"}
                    </button>
                </div>
            }
        </div>            
    }
}
