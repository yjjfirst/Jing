pub mod model;
pub mod node;

use web_sys::{EventTarget, FormData, SubmitEvent, HtmlFormElement, HtmlDialogElement};
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;
use yew_icons::{Icon, IconId};
use yewdux::prelude::*;

use node::{Node, NewNode};
use model::AclList;

use crate::components::header::Header;
use crate::components::dialog::Dialog;
use crate::components::action_buttons::ActionButtons;
use crate::models::Service;
use crate::store::Store;

#[derive(Clone, Routable, PartialEq)]
pub enum AclRoute {
    #[at("/acl")]
    Index,
    #[at("/acl/:id")]
    Get {id: usize},
}

#[derive(Clone, PartialEq, Properties)]
pub struct AclProps {
    pub acl: AclList,
    pub ondel: Callback<usize>
}

#[function_component]
pub fn AclListItem(props: &AclProps) -> Html {
    let acl = props.acl.clone();
    let nav = use_navigator().unwrap();
    let loc: Location = use_location().unwrap().clone();
    let (store,_) = use_store::<Store>();
    let ondel = props.ondel.clone();

    let dialog_ref: NodeRef = use_node_ref();
    let dd_ref = dialog_ref.clone();

    // clone small pieces to avoid moving the whole `acl` into multiple closures
    let acl_id_for_edit = acl.id as usize;
    let acl_id_for_confirm = acl.id as i32;
    let acl_name_for_display = acl.acl_name.clone();

    let onedit: Callback<MouseEvent> = Callback::from(move |_e|{
        nav.push(&AclRoute::Get {id: acl_id_for_edit});
    });

    let onconfirm: Callback<bool> = {
        let loc = loc.clone();
        let ondel = ondel.clone();
        let store_id = store.selected_domain_id;
        Callback::from(move |_e: bool|{
            let loc = loc.clone();
            let ondel = ondel.clone();
            let acl_id = acl_id_for_confirm;
            wasm_bindgen_futures::spawn_local(async move {
                let path = format!("{}/{}", loc.path(), acl_id);
                let _ = Service::delete(&path, store_id).await;
                ondel.emit(acl_id as usize);
            })
        })
    };

    let ondel_click: Callback<MouseEvent> = Callback::from(move |_e| {
        let d = dd_ref.cast::<HtmlDialogElement>().unwrap();
        d.show_modal().unwrap();
    });

    html! {
        <tr>
            <th>{acl_name_for_display.clone()}</th>
            <th class="flex justify-end">
                <div class="mr-1">
                    <div onclick={onedit} class="btn btn-square btn-outline btn-sm">
                        <Icon icon_id={IconId::LucideEdit}/>
                    </div>
                </div>
                <div>
                    <div onclick={ondel_click} class="btn btn-square btn-outline btn-sm">
                        <Icon icon_id={IconId::LucideTrash}/>
                    </div>
                </div>
            </th>
            <Dialog
                d_ref = {dialog_ref}
                title={"Warning!"}
                contents={format!("Are you sure to delete ACL: {}?", acl_name_for_display.clone())}
                {onconfirm}
                >
            </Dialog>
        </tr>
    }
}

#[function_component]
pub fn AclListPage() -> Html {
    let loc = use_location().unwrap().clone();
    let nav = use_navigator().unwrap();

    let (store,_) = use_store::<Store>();
    let acls: UseStateHandle<Vec<AclList>> = use_state(||vec![]);
    let acls_1 = acls.clone();
    let acls_2 = acls.clone();

    use_effect_with((), move|_|{
        let acls = acls_1.clone();
        let loc = loc.clone();
        let store = store.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched: Vec<AclList> = Service::index(loc.path(), store.selected_domain_id.clone()).await.unwrap_or_default();
            acls.set(fetched);
        });
    });

    let ondel: Callback<usize> = Callback::from(move|id: usize|{
        let acls = acls_2.clone();
        let filtered: Vec<AclList> = acls
            .iter()
            .filter(|g| {g.id as usize !=id})
            .map(|g|{g.clone()})
            .collect();

        acls.set(filtered);
    });

    let onadd: Callback<MouseEvent> = Callback::from(move|_e: MouseEvent|{
        nav.push(&AclRoute::Get {id: 0});
    });

    let acls_list: Vec<Html> = acls.iter().map(|g|{
        html! {
            <AclListItem acl={AclList {..g.clone()}} ondel={ondel.clone()}></AclListItem>
        }
    }).collect();

    html! {
        <div class="grow mr-2">
            <Header title="System -> ACL"></Header>
            <div class="divider my-1"></div>
            <table class="table table-zebra">
                <thead>
                    <tr>
                        <th>{"Name"}</th>
                    </tr>
                </thead>
                <tbody>
                {acls_list}
                </tbody>
            </table>
            <div class="flex flex-row-reverse pr-4">
                <div onclick={onadd} class="btn btn-square btn-outline btn-sm" >
                    <Icon icon_id={IconId::LucidePlus}/>
                </div>
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct AclDetailProps {
    pub id: usize,
}

#[function_component]
pub fn AclDetails(_props: &AclDetailProps) -> Html {
    let loc = use_location().unwrap();
    let nav = use_navigator().unwrap();
    let (store,_) = use_store::<Store>();

    let acl = use_state(|| AclList::new());
    let domain = store.selected_domain_id;
    {
        let acl = acl.clone();
        let loc = loc.clone();
        use_effect_with((), move |_|{
            let acl = acl.clone();
            let loc = loc.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let path = loc.path();
                let fetched = Service::get(&path, domain)
                    .await
                    .unwrap();

                acl.set(fetched);
            });
        });
    }

    let oncancel = {
        let nav = nav.clone();
        Callback::from(move |_e: MouseEvent| {
            nav.push(&AclRoute::Index);
        })
    };

    let form_onsubmit = {
        let acl = acl.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let target: Option<EventTarget> = e.target();
            let form = target.unwrap().dyn_into::<HtmlFormElement>().unwrap();
            let form_data = FormData::new_with_form(&form).unwrap();

            let acl_name = form_data.get("acl_name").as_string().unwrap_or_default();
            let acl_default = form_data.get("acl_default").as_string().unwrap_or_default();

            let acl_dto = AclList {
                id: acl.id,
                acl_name,
                acl_default,
                nodes: vec![],
            };

            let loc = loc.clone();
            let store = store.clone();
            let nav = nav.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match Service::post(loc.path(), store.selected_domain_id, acl_dto).await {
                    Ok(_) => {
                        nav.push(&AclRoute::Index);
                    }
                    Err(_) => {
                        web_sys::console::error_1(&"Failed to save ACL".into());
                    }
                }
            });
        })
    };

    let nodes_html: Html = acl.nodes.iter().map(|n|{
        html!{
            <Node
                cidr={n.cidr.clone()}
                node_type={n.node_type.clone()}>
            </Node>
        }
    }).collect();

    html!{
        <div class="grow mr-2">
            <Header title={format!("ACL -> {}", acl.acl_name.clone())}></Header>
            <div class="divider my-1"></div>
            <form class="w-full" onsubmit={form_onsubmit}>
                <div class="grid grid-cols-3 gap-1">
                    <label class="pbx-label">{"Name"}</label>
                    <input id="acl_name" name="acl_name" class="pbx-input" type="text" value={acl.acl_name.clone()} />
                    <label class="pbx-label">{"Default"}</label>
                    <input id="acl_default" name="acl_default" class="pbx-input" type="text" value={acl.acl_default.clone()} />
            <label class="pbx-label">{"Nodes"}</label>
            <div class="col-span-2">
            <div class="grid grid-cols-3 w-full gap-1">
                <div clsss="col-span-1">
                    {"CIDR"}
                </div>
                <div class="col-span-1">
                    {"TYPE"}
                </div>
                <div class="col-span-1">
                </div>
            </div>
        {nodes_html}
        <NewNode></NewNode>
        </div>
            </div>
                <ActionButtons {oncancel} />
            </form>

        </div>
    }
}

pub fn acl_switch(route: AclRoute) -> Html {
    match route {
        AclRoute::Index => html!{<AclListPage/>},
        AclRoute::Get { id } => html !{<AclDetails id={id}/>}
    }
}
