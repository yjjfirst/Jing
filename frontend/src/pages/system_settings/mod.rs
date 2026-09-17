pub mod model;

use std::collections::{BTreeMap, HashMap};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use serde::{Serialize, Deserialize};

use crate::components::header::Header;
use crate::components::action_buttons::ActionButtons;
use crate::models::Service;
use crate::store::Store;

use model::{SystemSetting};

#[derive(Clone, Routable, PartialEq)]
pub enum SystemSettingsRoute {
    #[at("/system-settings")]
    Index,
}

#[component]
pub fn SystemSettings() -> Html {
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

    let handle_submit = {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct Setting {
            pub setting_key: String,
            pub setting_value: String,
        }

        let store = store.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let store = store.clone();
            let form = e.target_unchecked_into::<web_sys::HtmlFormElement>();
            let form_data = web_sys::FormData::new_with_form(&form).unwrap();
            let mut settings_map: HashMap<String, Vec<Setting>> = HashMap::new();
            let entries = js_sys::try_iter(&form_data).unwrap().unwrap();
            for entry in entries {
                let entry = entry.unwrap();
                let array = js_sys::Array::from(&entry);
                let key = array.get(0).as_string().unwrap();
                let (section, setting_key) = key.split_once("-").unwrap();
                let value = array.get(1).as_string().unwrap();
                settings_map.entry(section.to_string())
                        .or_insert_with(Vec::new)
                        .push(Setting {
                            setting_key: setting_key.to_string(),
                            setting_value: value,
                });
            }
            wasm_bindgen_futures::spawn_local(async move {
                Service::post("/system-settings", store.selected_domain_id, settings_map)
                    .await
                    .unwrap();
            });
        })
    };

    html! {
        <div class="grow mr-2">
            <Header title="System Settings"></Header>
            <div class="divider my-1"></div>
            <form onsubmit={handle_submit.clone()}>
                for (section, settings) in system_settings.iter() {
                    <fieldset class="fieldset bg-base-200 border-base-300 rounded-box border p-4 text-base">
                        <legend class="fieldset-legend">{section.clone()}</legend>
                        <div class="grid grid-cols-3 gap-1">
                            for setting in settings {
                                <label class="pbx-label">{setting.setting_key.clone()}</label>
                                <input 
                                    type="text" 
                                    class="pbx-input" 
                                    name={format!("{}-{}", section, setting.setting_key.clone())}
                                    value={setting.setting_value.clone()} 
                                />
                            }
                        </div>
                    </fieldset>                 
                }
                <ActionButtons  has_cancel={false}/>
            </form>
        </div>
    }
}

pub fn system_settings_switch(route: SystemSettingsRoute) -> Html {
    match route {
        SystemSettingsRoute::Index => html!{
            <SystemSettings />
        }
    }
}

