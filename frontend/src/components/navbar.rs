use yew::prelude::*;
use yewdux::prelude::*;
use gloo_net::http::Request;
use crate::store::{Store, set_username, set_is_authenticated, set_selected_domain_id, set_selected_domain_name};
use yew_icons::{Icon, IconId};
use crate::app::Route;
use yew_router::prelude::*;

use crate::models::API_BASE;

#[function_component]
pub fn Navbar() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let handle_logout = {
        Callback::from(move |_|{
            let dispatch = dispatch.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("{}/logout", API_BASE);
                let req = Request::post(&url).body("").unwrap();
                let dispatch = dispatch.clone();
                let res = req.send().await.unwrap();
                if res.ok() {
                    set_is_authenticated(false, dispatch.clone());
                    set_username("".to_string(), dispatch);
                }
            });
        })
    };

    html! {
        <div class="flex justify-end grow items-center ml-4 mr-4">
            <DomainComponent/>
            <div class="flex items-center">
                <p>{store.username.clone()}</p>
                <div class="btn btn-ghost btn-sm" onclick={handle_logout}>
                    <Icon icon_id={IconId::LucideLogOut}/>
                </div>
            </div>
        </div>
    }
}

#[function_component]
pub fn DomainComponent() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let nav = use_navigator().unwrap();

    html!{
        <div class="navbar bg-base-100 shadow-sm">
            <div class="flex-1">
              	<a class="btn btn-ghost text-xl"></a>
            </div>
            <div class="flex gap-2">
           		<div class="dropdown dropdown-end">
             		<div tabindex="0" role="button" class="btn btn-ghost">
                        <span>{store.selected_domain_name.clone()}</span>
             		</div>
             		<ul
               			tabindex="-1"
               			class="menu menu-sm dropdown-content bg-base-100 rounded-box z-1 mt-3 w-36 p-2 shadow">
                        {
                            store.domains.clone().into_iter().map(move |d| {
                                let dd = d.clone();
                                let dispatch = dispatch.clone();
                                let nav = nav.clone();
                                html!{
                                    <li class="text-xl">
                                        <a onclick={Callback::from( move |_|{
                                            let dispatch = dispatch.clone();
                                            let d = dd.clone();
                                            let nav = nav.clone();
                                            set_selected_domain_id(d.id.clone(), dispatch.clone());
                                            set_selected_domain_name(d.domain_name.clone(), dispatch.clone());
                                            nav.push(&Route::Dashboard);
                                        })}>
                                            {d.domain_name.clone()}
                                        </a>
                                    </li>}
                            }).collect::<Html>()
                        }
             		</ul>
              	</div>
          	</div>
      	</div>
    }
}
