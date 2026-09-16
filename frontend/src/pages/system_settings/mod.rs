pub mod model;

use std::collections::BTreeMap;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::components::header::Header;
use crate::models::Service;
use crate::store::Store;

use model::{SystemSetting};

#[derive(Clone, Routable, PartialEq)]
pub enum SystemSettingsRoute {
    #[at("/system-settings")]
    Index,
}

#[component]
pub fn SystemSettingsPage() -> Html {
    let system_settings: UseStateHandle<BTreeMap<String, Vec<SystemSetting>>> = 
        use_state(|| BTreeMap::new());
    let (store, _) = use_store::<Store>();
    {
        let system_settings = system_settings.clone();
        let store = store.clone();
        use_effect_with((), move |_| {
            let system_settings = system_settings.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_settings: BTreeMap<String, Vec<SystemSetting>> = Service::get("/system-settings", store.selected_domain_id)
                    .await
                    .unwrap();
                system_settings.set(fetched_settings);
            });
        });
    }

    html! {
        <div class="grow mr-2">
            <Header title="System Settings"></Header>
            <div class="divider my-1"></div>
            for (section, settings) in system_settings.iter() {
                <fieldset class="fieldset bg-base-200 border-base-300 rounded-box border p-4 text-base">
                    <legend class="fieldset-legend">{section.clone()}</legend>
                    for setting in settings {
                        <div class="grid grid-cols-3 gap-1">
                            <label class="pbx-label">{setting.setting_key.clone()}</label>
                            <input type="text" class="pbx-input" placeholder={setting.setting_value.clone()} />
                        </div>
                    }
                </fieldset>                 
            }
        </div>
    }
}

pub fn system_settings_switch(route: SystemSettingsRoute) -> Html {
    match route {
        SystemSettingsRoute::Index => html!{
            <SystemSettingsPage />
        }
    }
}

