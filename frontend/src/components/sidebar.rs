use crate::app::Route;
use wasm_bindgen::JsCast;
use web_sys::HtmlAnchorElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct SidebarMenuItemPros {
    pub caption: String,
    pub route: Route,
    pub onclick: Callback<String>,
    pub selected: bool,
}

#[function_component]
pub fn SideBar() -> Html {
    html! {
        <div class="w-60 flex-row">
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
        },
        SidebarMenuItemPros {
            caption: "Ringing Group".to_string(),
            route: Route::RingingGroupsRoot,
            onclick: onclick.clone(),
            selected: false,
        },
    ];

    let conn_items = vec![
        SidebarMenuItemPros {
            caption: "Gateway".to_string(),
            route: Route::GatewayRoot,
            onclick: onclick.clone(),
            selected: false,
        },
        SidebarMenuItemPros {
            caption: "Outbound Route".to_string(),
            route: Route::RingingGroupsRoot,
            onclick: onclick.clone(),
            selected: false,
        },
        SidebarMenuItemPros {
            caption: "Inbound Route".to_string(),
            route: Route::RingingGroupsRoot,
            onclick: onclick.clone(),
            selected: false,
        },
    ];

    let report_items = vec![SidebarMenuItemPros {
        caption: "CDR".to_string(),
        route: Route::RingingGroupsRoot,
        onclick: onclick.clone(),
        selected: false,
    }];
    let sys_items = vec![SidebarMenuItemPros {
        caption: "Firewall".to_string(),
        route: Route::RingingGroupsRoot,
        onclick: onclick.clone(),
        selected: false,
    }];
    html! {
          <div class="flex flex-col">
          <ul class="menu bg-base-200 w-60">
            <SidebarMenuItem
                route={Route::Dashboard}
                caption={"Dashboard"}
                onclick={onclick.clone()}
                selected = {*selected == "Dashboard"}>
            </SidebarMenuItem>
            <li>
            <details>
              <summary>{"Applications"}</summary>
              <ul>
              {app_items.iter().map(|i|{
                  html! {
                      <SidebarMenuItem
                          route={i.route.clone()}
                          caption={i.caption.clone()}
                          onclick={i.onclick.clone()}
                          selected = {*selected == i.caption.clone()}>
                      </SidebarMenuItem>
                  }
              }).collect::<Vec<Html>>()}
              </ul>
            </details>
          </li>
          <li>
          <details>
            <summary>{"Connection"}</summary>
            <ul>
            {conn_items.iter().map(|i|{
              html! {
                  <SidebarMenuItem
                      route={i.route.clone()}
                      caption={i.caption.clone()}
                      onclick={i.onclick.clone()}
                      selected = {*selected == i.caption.clone()}>
                  </SidebarMenuItem>
              }
          }).collect::<Vec<Html>>()}

            </ul>
          </details>
        </li>
        <li>
        <details>
          <summary>{"Reports"}</summary>
          <ul>
          {report_items.iter().map(|i|{
              html! {
                  <SidebarMenuItem
                      route={i.route.clone()}
                      caption={i.caption.clone()}
                      onclick={i.onclick.clone()}
                      selected = {*selected == i.caption.clone()}>
                  </SidebarMenuItem>
              }
          }).collect::<Vec<Html>>()}
          </ul>
        </details>
      </li>
      <li>
      <details>
        <summary>{"System"}</summary>
        <ul>
        {sys_items.iter().map(|i|{
          html! {
              <SidebarMenuItem
                  route={i.route.clone()}
                  caption={i.caption.clone()}
                  onclick={i.onclick.clone()}
                  selected = {*selected == i.caption.clone()}>
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

    let onclick = Callback::from(move |e: MouseEvent| {
        let a: HtmlAnchorElement = e.target().unwrap().dyn_into::<HtmlAnchorElement>().unwrap();
        props_onclick.emit(a.rel().to_string());
        nav.push(&p.route);
    });

    let mut classes = vec![""];
    if props.selected {
        classes.push("active");
    }

    html! {
        <li>
            <a {onclick} class={classes!(classes)} rel={props.caption.clone()}>
                { props.caption.clone() }
            </a>
        </li>
    }
}
