use crate::app::Route;
use yew::prelude::*;
use yew_router::prelude::*;
use yew_icons::{Icon, IconData};


#[derive(Clone, Properties, PartialEq)]
pub struct SidebarMenuItemPros {
    pub caption: String,
    pub route: Route,
    pub onclick: Callback<String>,
    pub selected: bool,
    pub icon: Option<IconData>,
}

#[function_component]
pub fn SideBar() -> Html {
    html! {
        <div class="w-60 flex-row rounded-box">
            <SidebarMenu/>
        </div>
    }
}

#[function_component]
pub fn SidebarMenu() -> Html {
    let selected = use_state(|| "".to_string());
    let onclick = {
        let selected = selected.clone();
        Callback::from(move |caption: String| {
            selected.set(caption);
        })
    };

    let app_items = vec![
        SidebarMenuItemPros {
            caption: "User".to_string(),
            route: Route::ExtensionRoot,
            onclick: onclick.clone(),
            selected: false,
            icon: None,
        },
        SidebarMenuItemPros {
            caption: "Ring Group".to_string(),
            route: Route::RingGroupsRoot,
            onclick: onclick.clone(),
            selected: false,
            icon: None,
        },
        SidebarMenuItemPros {
            caption: "Sound".to_string(),
            route: Route::SoundRoot,
            onclick: onclick.clone(),
            selected: false,
            icon: None,
        },
        SidebarMenuItemPros {
            caption: "Conference".to_string(),
            route: Route::ConferenceRoot,
            onclick: onclick.clone(),
            selected: false,
            icon: None,
        },
        SidebarMenuItemPros {
            caption: "IVR".to_string(),
            route: Route::IvrRoot,
            onclick: onclick.clone(),
            selected: false,
            icon: None,
        },
        SidebarMenuItemPros {
            caption: "Call Center".to_string(),
            route: Route::CallcenterRoot,
            onclick: onclick.clone(),
            selected: false,
            icon: None,
        },
    ];

    let conn_items = vec![
        SidebarMenuItemPros {
            caption: "Gateway".to_string(),
            route: Route::GatewayRoot,
            onclick: onclick.clone(),
            selected: false,
            icon: None,
        },
        SidebarMenuItemPros {
            caption: "Outbound Route".to_string(),
            route: Route::OutboundRoot,
            onclick: onclick.clone(),
            selected: false,
            icon: None,
        },
        SidebarMenuItemPros {
            caption: "Inbound Route".to_string(),
            route: Route::InboundRoot,
            onclick: onclick.clone(),
            selected: false,
            icon: None,
        },
    ];

    let report_items = vec![SidebarMenuItemPros {
        caption: "CDR".to_string(),
        route: Route::Cdr,
        onclick: onclick.clone(),
        selected: false,
        icon: None,
    }];
    let sys_items = vec![
        SidebarMenuItemPros {
            caption: "ACL".to_string(),
            route: Route::AclRoot,
            onclick: onclick.clone(),
            selected: false,
            icon: None,
        },
        SidebarMenuItemPros {
            caption: "SIP Profile".to_string(),
            route: Route::ProfileRoot,
            onclick: onclick.clone(),
            selected: false,
            icon: None,
        },
        SidebarMenuItemPros {
            caption: "Sound File".to_string(),
            route: Route::SoundFileRoot,
            onclick: onclick.clone(),
            selected: false,
            icon: None,
        },
        SidebarMenuItemPros {
            caption: "System Settings".to_string(),
            route: Route::SystemSettings,
            onclick: onclick.clone(),
            selected: false,
            icon: None,
        },
        SidebarMenuItemPros {
            caption: "Firewall".to_string(),
            route: Route::Firewall,
            onclick: onclick.clone(),
            selected: false,
            icon: None,
        },
    ];
    html! {
          <div class="flex flex-col">
          <ul class="menu bg-base-200 w-60">
            <SidebarMenuItem
                route={Route::Dashboard}
                caption={"Dashboard"}
                onclick={onclick.clone()}
                selected = {*selected == "Dashboard"}
                icon={Some(IconData::LUCIDE_PIE_CHART)}>
            </SidebarMenuItem>
            <li>
            <details>
              <summary><Icon data={IconData::LUCIDE_PHONE}/>{"Applications"}</summary>
              <ul>
              {app_items.iter().map(|i|{
                  html! {
                      <SidebarMenuItem
                          route={i.route.clone()}
                          caption={i.caption.clone()}
                          onclick={i.onclick.clone()}
                          selected = {*selected == i.caption.clone()}
                          icon ={i.icon.clone()}
                          >
                      </SidebarMenuItem>
                  }
              }).collect::<Vec<Html>>()}
              </ul>
            </details>
          </li>
          <li>
          <details>
            <summary><Icon data={IconData::LUCIDE_ARROW_UP_DOWN}/>{"Connection"}</summary>
            <ul>
            {conn_items.iter().map(|i|{
              html! {
                  <SidebarMenuItem
                      route={i.route.clone()}
                      caption={i.caption.clone()}
                      onclick={i.onclick.clone()}
                      selected = {*selected == i.caption.clone()}
                      icon = {i.icon.clone()}>
                  </SidebarMenuItem>
              }
          }).collect::<Vec<Html>>()}

            </ul>
          </details>
        </li>
      <li>
      <details>
        <summary><Icon data={IconData::LUCIDE_SETTINGS}/>{"System"}</summary>
        <ul>
        {sys_items.iter().map(|i|{
            html! {
                <SidebarMenuItem
                    route={i.route.clone()}
                    caption={i.caption.clone()}
                    onclick={i.onclick.clone()}
                    selected = {*selected == i.caption.clone()}
                    icon ={i.icon.clone()}>
                </SidebarMenuItem>
            }
        }).collect::<Vec<Html>>()}
        </ul>
      </details>
      </li>
        <li>
        <details>
          <summary><Icon data={IconData::LUCIDE_FILE_TEXT}/>{"Reports"}</summary>
          <ul>
          {report_items.iter().map(|i|{
              html! {
                  <SidebarMenuItem
                      route={i.route.clone()}
                      caption={i.caption.clone()}
                      onclick={i.onclick.clone()}
                      selected = {*selected == i.caption.clone()}
                      icon ={i.icon.clone()}>
                  </SidebarMenuItem>
              }
          }).collect::<Vec<Html>>()}
          </ul>
        </details>
      </li>
    </ul>
    </div>
      }
}

#[function_component]
pub fn SidebarMenuItem(props: &SidebarMenuItemPros) -> Html {
    let nav = use_navigator().unwrap();
    let props_onclick = props.onclick.clone();
    let p = props.clone();

    let onclick = Callback::from(move |_: MouseEvent| {
        props_onclick.emit(p.caption.clone());
        nav.push(&p.route);
    });

    html! {
        <li>
            <a {onclick} class={ if props.selected {"menu-active"} else {""}}>
                if let Some(icon_data) = &props.icon {
                    <Icon data={icon_data.clone()}/>
                }
                { props.caption.clone() }
            </a>
        </li>
    }
}
