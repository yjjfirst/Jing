use yew::prelude::*;
use yewdux::prelude::*;
use crate::store::{Store, set_selected_domain_id, set_selected_domain_name};
use yew_icons::{Icon, IconId};
use crate::app::Route;
use yew_router::prelude::*;

#[function_component]
pub fn Banner() -> Html {
    let (store, _) = use_store::<Store>();

    html! {
        <div class="flex justify-end grow items-center ml-1 mr-1">
            <DomainComponent/>
            <div class="flex items-center">
                <div class="btn btn-ghost btn-sm">
                    <Icon icon_id={IconId::LucideLogOut}/>
                </div>
                <p>{store.username.clone()}</p>
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
