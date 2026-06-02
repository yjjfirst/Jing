pub mod model;

use web_sys::{EventTarget, FormData, SubmitEvent, HtmlFormElement, HtmlDialogElement};
use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yew_icons::{Icon, IconId};
use wasm_bindgen::JsCast;

use crate::components::header::Header;
use model::{RingGroup};
use crate::models::Service;
use crate::store::{alert_info, alert_error, Store};
use crate::components::input::Input;
use crate::components::label::Label;
use crate::components::select::Select;
use crate::components::select_multi::MultiSelect;
use crate::components::action_buttons::ActionButtons;
use crate::pages::user::model::User;
use crate::components::dialog::Dialog;

#[derive(Clone, Routable, PartialEq)]
pub enum RingGroupsRoute {
    #[at("/ring-group")]
    Index,
    #[at("/ring-group/:id")]
    Get {id: usize},
}

#[derive(Clone, PartialEq, Properties)]
pub struct RingGroupDetailsProps {
    #[prop_or(0)]
    pub id: usize,
}

#[derive(Clone, PartialEq, Properties)]
pub struct RingGroupListItemProps {
    pub id: usize,
    pub group_id: String,
    pub name: String,
    pub ondel: Callback<usize>
}

#[function_component]
pub fn RingGroupList() -> Html {
    let loc = use_location().unwrap().clone();
    let (store,_) = use_store::<Store>();
    let ring_groups: UseStateHandle<Vec<RingGroup>> = use_state(||vec![]);
    let groups = ring_groups.clone();
    let groups_1 = ring_groups.clone();

    let nav = use_navigator().unwrap();

    use_effect_with((), move |_| {
        let groups = groups.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_groups: Vec<RingGroup> =
                Service::index(loc.path(), store.selected_domain_id.clone())
                    .await
                    .unwrap();
            groups.set(fetched_groups);
        });
    });

    let ondel = Callback::from(move |id: usize|{
        let groups = groups_1.clone();
        let filtered: Vec<RingGroup> = groups
            .iter()
            .filter(|g|id != g.id)
            .map(|g|{g.clone()})
            .collect();

        groups.set(filtered);
    });

    let groups: Vec<Html> = ring_groups.iter().map(|g| html! {
        <RingGroupListItem
            ondel={ondel.clone()}
            id={g.id}
            group_id={g.group_id.clone()}
            name={g.name.clone()}>
        </RingGroupListItem>
    }).collect();

    let onadd: Callback<MouseEvent> = Callback::from(move|_e: MouseEvent| {
        nav.push(&RingGroupsRoute::Get {id: 0});
    });

    html! {
        <div class="grow mr-2">
            <Header title="Application -> Ring Group"></Header>
            <div class="divider my-1"></div>
            <table class="table table-zebra">
                <thead>
                    <th>{"Extension"}</th>
                    <th>{"Name"}</th>
                </thead>
                <tbody>
                    {groups}
                </tbody>
            </table>
            <div class="flex flex-row-reverse pr-4">
                <div onclick={onadd} class="btn btn-square btn-outline btn-sm">
                    <Icon icon_id={IconId::LucidePlus}/>
                </div>
            </div>
        </div>
    }

}

#[function_component]
pub fn RingGroupListItem(props: &RingGroupListItemProps) -> Html {
    let props = props.clone();
    let id = props.id;
    let nav = use_navigator().unwrap();
    let loc: Location = use_location().unwrap().clone();
    let (store,_) = use_store::<Store>();
    let ondel = props.ondel.clone();

    let dialog_ref: NodeRef = use_node_ref();
    let dd_ref = dialog_ref.clone();

    let onconfirm: Callback<bool> = Callback::from(move |_e: bool|{
        let loc = loc.clone();
        let store = store.clone();
        let ondel = ondel.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let path = format!("{}/{}", loc.path(), id);
            Service::delete(&path, store.clone().selected_domain_id)
                .await
                .unwrap();
            ondel.emit(id);
        })
    });

    let onedit: Callback<MouseEvent> = Callback::from(move|_e: MouseEvent| {
        let nav = nav.clone();
        nav.push(&RingGroupsRoute::Get {id});
    });

    let ondel: Callback<MouseEvent> = Callback::from(move|_e: MouseEvent| {
        let d = dd_ref.cast::<HtmlDialogElement>().unwrap();
        d.show_modal().unwrap();
    });

    html! {
        <tr>
            <th>{props.group_id.clone()}</th>
            <th>{props.name}</th>
            <th class="flex justify-end">
                <div onclick={onedit} class="mr-1">
                    <div class="btn btn-square btn-outline btn-sm">
                        <Icon icon_id={IconId::LucideEdit}/>
                    </div>
                </div>
                <div onclick={ondel}>
                    <div class="btn btn-square btn-outline btn-sm">
                        <Icon icon_id={IconId::LucideTrash}/>
                    </div>
                </div>
            </th>
            <Dialog
                d_ref = {dialog_ref}
                title={"Warning!"}
                contents={format!("Are you sure to delete Ringing Group: {}?", props.group_id.clone())}
                {onconfirm}
                >
            </Dialog>
        </tr>
    }
}

#[function_component]
pub fn RingGroupDetailComponent(props: &RingGroupDetailsProps) -> Html {
    let(store, dispatch) = use_store::<Store>();
    let id = props.id;
    let group: UseStateHandle<RingGroup> = use_state(||RingGroup::new_empty());

    let loc = use_location().unwrap();
    let location = loc.clone();
    let nav = use_navigator().unwrap();

    let users: UseStateHandle<Vec<String>> = use_state(||vec![]);
    {
        let group = group.clone();
        let users = users.clone();
        let store = store.clone();
        use_effect_with((), move |_| {
            let group = group.clone();
            let users = users.clone();
            let loc = location.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_group: RingGroup =
                    Service::get(loc.path(), store.clone().selected_domain_id)
                    .await
                    .unwrap();
                gloo_console::log!("{:?}", fetched_group.ring_time);
                group.set(fetched_group);

                let fetched_users = User::list(store.selected_domain_id).await;
                users.set(fetched_users);
            });
        });
    }

    let options: Vec<String> = vec![
        String::from("simultaneous"),
        String::from("sequential")
    ];

    let form_oncancel = {
        let nav = nav.clone();
        Callback::from(move|_| {
            nav.push(&RingGroupsRoute::Index);
        })
    };

    let form_onsubmit = {
        let dispatch: Dispatch<Store> = dispatch.clone();
        let store = store.clone();
        let loc = loc.clone();
        let nav = nav.clone();

        Callback::from( move| event: SubmitEvent|{
            let nav = nav.clone();
            let dispatch: Dispatch<Store> = dispatch.clone();
            let store = store.clone();
            let loc = loc.clone();

            let target: Option<EventTarget> = event.target();
            let form = target.unwrap().dyn_into::<HtmlFormElement>().unwrap();
            let form_data = FormData::new_with_form(&form).unwrap();
            let members = form_data.get_all("members");

            let group = RingGroup::new(
                id,
                form_data.get("name").as_string().unwrap(),
                form_data.get("extension").as_string().unwrap(),
                form_data.get("description").as_string().unwrap(),
                store.selected_domain_id,
                form_data.get("ring-time").as_string().unwrap().parse::<i32>().unwrap(),
                form_data.get("ring-strategy").as_string().unwrap(),
                members.iter().map(|m|{
                    m.as_string().unwrap()
                }).filter(|m|{
                    !m.eq("")
                }).collect::<Vec<String>>()
            );

            wasm_bindgen_futures::spawn_local(async move {
                let dispatch = dispatch.clone();

                match Service::post(loc.path(), store.selected_domain_id, group).await {
                    Ok(_) => {
                        alert_info("Update inbound route successfully.".to_string(), dispatch);
                    }
                    Err(_) => {
                        alert_error("Update inbound route failed.".to_string(), dispatch);
                    }
                }
                nav.push(&RingGroupsRoute::Index);
            });

            event.prevent_default();
        })
    };

    html! {
        <div class="grow mr-2">
            <Header title= {format!("Ring Group: {}", group.group_id.clone())}></Header>
            <div class="divider my-1"></div>
            <form class="w-full" onsubmit={form_onsubmit}>
            <div class="grid grid-cols-3 gap-1">
                <Label hidden={group.group_id.clone() != ""}>{"Extension"}</Label>
                <Input
                    value={group.group_id.clone()}
                    id="extension"
                    hidden={group.group_id.clone() != ""}
                />
                <Label>{"Name"}</Label>
                <Input
                    value={group.name.clone()}
                    id="name"
                    />
                <Label>{"Description"}</Label>
                <Input
                    value={group.description.clone()}
                    id="description"
                    />
                <Label>{"Ringing Time"}</Label>
                <Input
                    value={group.ring_time.to_string()}
                    input_type="number"
                    id="ring-time"
                    />
                <Label>{"Ring Stragegy"}</Label>
                <Select
                    {options}
                    selected = {group.ring_strategy.to_string()}
                    id="ring-strategy">
                </Select>
                <Label>{"Members"}</Label>
                <MultiSelect
                    exists = {group.members.clone()}
                    all = {users.iter().map(|u|u.to_string()).collect::<Vec<String>>()}
                    >
                </MultiSelect>
                </div>
                <ActionButtons oncancel={form_oncancel}/>
            </form>
        </div>
    }
}

pub fn ringgroups_switch(route: RingGroupsRoute) -> Html {
    match route {
        RingGroupsRoute::Index => html!{ <RingGroupList />},
        RingGroupsRoute::Get { id } => html!{<RingGroupDetailComponent id={id}/>}
    }
}
