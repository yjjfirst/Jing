mod model;

use std::collections::HashMap;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{EventTarget, FormData, SubmitEvent, HtmlFormElement};
use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yew_icons::{Icon, IconData};

use crate::components::header::Header;
use crate::components::action_buttons::ActionButtons;
use crate::components::param_input::ParamInput;

use crate::models::Service;
use model::{Profile, Param};
use crate::store::{alert_info, alert_error, Store};

#[derive(Clone, Routable, PartialEq)]
pub enum ProfileRoute {
    #[at("/profile")]
    Index,
    #[at("/profile/:id")]
    Get { id: usize },
}

#[function_component]
pub fn ProfileList() -> Html {
    let loc = use_location().unwrap().clone();
    let (store, _) = use_store::<Store>();
    let profiles: UseStateHandle<Vec<Profile>> = use_state(|| vec![]);
    let profiles_c = profiles.clone();

    use_effect_with((), move |_|{
        let profiles = profiles_c.clone();
        let loc = loc.clone();
        let store = store.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched: Vec<Profile> = Service::index(loc.path(), store.selected_domain_id)
                .await
                .unwrap();
            profiles.set(fetched);
        });
    });

    let list: Vec<Html> = profiles.iter().map(|p|{
        html!{
            <tr>
                <th>{p.name.clone()}</th>
                <th class="flex justify-end">
                    <div class="mr-1">
                        <Link<ProfileRoute> to={ProfileRoute::Get { id: p.id }} classes="btn btn-square btn-outline btn-sm">
                            <Icon data={IconData::LUCIDE_EDIT}/>
                        </Link<ProfileRoute>>
                    </div>
                </th>
            </tr>
        }
    }).collect();

    html!{
        <div class="grow mr-2">
            <Header title="System -> SIP Profile"></Header>
            <div class="divider my-1"></div>
            <table class="table table-zebra">
                <thead>
                    <tr>
                        <th>{"Name"}</th>
                    </tr>
                </thead>
                <tbody>
                { list }
                </tbody>
            </table>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ProfileDetailProps {
    pub id: usize,
}

#[function_component]
pub fn ProfileDetails(props: &ProfileDetailProps) -> Html {
    let id = props.id;
    let profile = use_state(|| Profile::new());
    let (store, dispatch) = use_store::<Store>();
    let loc = use_location().unwrap();
    let nav = use_navigator().unwrap();

    {
        let store = store.clone();
        let profile = profile.clone();
        let loc = loc.clone();
        use_effect_with((), move |_|{
            let profile = profile.clone();
            let loc = loc.clone();
            let store = store.clone();
            spawn_local(async move {
                let fetched: Profile = Service::get(loc.path(), store.selected_domain_id).await.unwrap_or(Profile::new());
                profile.set(fetched);
            });
        });
    }

    let form_oncancel = {
        let nav = nav.clone();
        Callback::from(move |_| {
            let nav = nav.clone();
            nav.push(&ProfileRoute::Index);
        })
    };

    let form_onsubmit = {
        let nav = nav.clone();
        let profile= profile.clone();
        let dispatch = dispatch.clone();
        let loc = loc.clone();
        let store = store.clone();
        Callback::from(move |e: SubmitEvent| {
            let nav = nav.clone();
            let store = store.clone();
            let target: Option<EventTarget> = e.target();
            let form = target.unwrap().dyn_into::<HtmlFormElement>().unwrap();
            let form_data = FormData::new_with_form(&form).unwrap();
            let dispatch = dispatch.clone();
            let loc = loc.clone();

            e.prevent_default();

            let params = profile.param_helps.iter()
                .filter(|h| !h.readonly && !Param::ignored(&h.name, &profile.params))
                .map(|h|{
                    let key = h.name.clone();
                    match profile.params.get(&key) {
                        Some(p) => {
                            let mut param = p.clone();
                            param.value = form_data.get(&key).as_string().unwrap_or_default();
                            (key, param)
                        },
                        None => {
                            let mut param = Param::new();
                            param.name = key.clone();
                            param.value = form_data.get(&key).as_string().unwrap_or_default();
                            (key.clone(), param)
                        }
                    }
                }).collect::<HashMap<String, Param>>();
            let updated = Profile {
                id,
                name: form_data.get("name").as_string().unwrap_or_default(),
                param_helps: vec![],
                params,
            };

            wasm_bindgen_futures::spawn_local(async move {
                let dispatch = dispatch.clone();
                let loc = loc.clone();
                match Service::post(loc.path(), store.selected_domain_id, updated).await {
                    Ok(_) => alert_info("Update profile successfully.".to_string(), dispatch),
                    Err(_) => alert_error("Update profile failed.".to_string(), dispatch),
                }
                nav.push(&ProfileRoute::Index);
            });
        })
    };

    html!{
        <div class="grow mr-2">
            <Header title={format!("Profile: {}", profile.name.clone())}></Header>
            <div class="divider my-1"></div>
            <form class="w-full" onsubmit={form_onsubmit}>
                <div class="grid grid-cols-3 gap-1">
                    <label class="pbx-label">{"name"}</label>
                    <input class="pbx-input" value={profile.name.clone()} name="name" />
                    { for profile.param_helps
                        .iter()
                        .filter(|h| !h.readonly &&!Param::ignored(&h.name, &profile.params))
                        .map(|h| {
                            html!{
                                <>
                                    <label class="pbx-label">{h.name.clone()}</label>
                                    <ParamInput
                                        value={Param::get(&h.name, &profile.params)}
                                        name={h.name.clone()}
                                        range_text={h.range_text.clone()}
                                        help_text={h.help_text.clone()}
                                    />
                                </>
                            }
                    })}
                </div>
                <ActionButtons oncancel={form_oncancel}/>
            </form>
        </div>
    }
}

pub fn profile_switch(route: ProfileRoute) -> Html {
    match route {
        ProfileRoute::Index => html!{<ProfileList />},
        ProfileRoute::Get { id } => html!{<ProfileDetails id={id}/>}
    }
}
