use serde::{Deserialize, Serialize};
use yewdux::prelude::*;
use crate::models::domain::Domain;
use crate::components::alert::AlertType;

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct AlertInput {
    pub show_alert: bool,
    pub alert_message: String,
    pub alert_type: AlertType,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Store)]
pub struct Store {
    pub alert_input: AlertInput,
    pub selected_domain: usize,
    pub domains: Vec<Domain>,
    pub is_authenticated: bool,
}

pub fn alert_info(message: String, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.alert_input = AlertInput {
            alert_message: message,
            show_alert: true,
            alert_type: AlertType::INFO,
        };
    })
}

pub fn alert_error(message: String, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.alert_input = AlertInput {
            alert_message: message,
            show_alert: true,
            alert_type: AlertType::ERROR,
        };
    })
}
pub fn hide_alert(dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.alert_input.show_alert = false;
    })
}

pub fn select_domain(domain: usize, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move|store| {
        store.selected_domain = domain;
    })
}

pub fn set_domains(domains: Vec<Domain>, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move|store: &mut Store| {
        store.domains = domains;
    })
}

pub fn set_is_authenticated(is_authenticated: bool, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store|{
        store.is_authenticated = is_authenticated;
    })
}
