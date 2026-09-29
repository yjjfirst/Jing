use std::collections::HashMap;
use actix_web::{web, Responder};
use jlib::system_setting;
use serde::{Serialize, Deserialize};
use super::Status;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostSetting {
    pub setting_key: String,
    pub setting_value: String,
}

pub fn system_setting_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("")
                .route(web::get().to(index))
                .route(web::post().to(post)));
}

async fn index() -> impl Responder {
    let sections = system_setting::list_sections().unwrap(); 
    let mut settings_map = HashMap::new();
    for section in sections {
        let s = system_setting::get_settings_by_section(&section).unwrap();
        settings_map.insert(section, s);
    };

     web::Json(settings_map)
}

async fn post(body: web::Json<HashMap<String, Vec<PostSetting>>>) -> impl Responder {
    let settings = body.into_inner();
    for (section, settings) in settings {
        for s in settings {
            system_setting::update(&section, &s.setting_key, &s.setting_value).unwrap();
        }
    }
    web::Json(Status {status: "Ok".to_string()})
}
