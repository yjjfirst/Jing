use gloo_timers::callback::Timeout;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yewdux::prelude::use_store;
use crate::store::{hide_alert, Store};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub enum AlertType {
    #[default]
    INFO,
    ERROR
}

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub message: String,
    pub delay_ms: u32,
    pub alert_type: AlertType
}

#[function_component(AlertComponent)]
pub fn alert_component(props: &Props) -> Html {
    let (store, dispatch) = use_store::<Store>();
    let show_alert = store.alert_input.show_alert;
    let alert_t: AlertType = store.alert_input.alert_type.clone();

    use_effect_with((show_alert, dispatch.clone(), props.delay_ms), move |(show_alert, dispatch, delay_ms)| {
            let cloned_dispatch = dispatch.clone();
            if *show_alert {
                let handle =
                    Timeout::new(*delay_ms, move || hide_alert(cloned_dispatch)).forget();
                let clear_handle = move || {
                    web_sys::Window::clear_timeout_with_handle(
                        &web_sys::window().unwrap(),
                        handle.as_f64().unwrap() as i32,
                    );
                };

                Box::new(clear_handle) as Box<dyn FnOnce()>
            } else {
                Box::new(|| {}) as Box<dyn FnOnce()>
            }
        }
    );
    let alert_class = match alert_t {
        AlertType::INFO => {
            classes!("alert", "alert-info")
        },
        AlertType::ERROR => {
            classes!("alert", "alert-error")
        }
    };
    html! {
    <div class={format!("toast toast-top toast-center {}", if show_alert { "" } else { "hidden" })}>
        <div class={alert_class}>
            <span>{props.message.clone()}</span>
        </div>
    </div>
    }
}
