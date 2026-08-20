use actix_web::{web, Responder};
use fs_lib::system_setting;
use serde::{Serialize, Deserialize};
use super::Status;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemSetting {
    pub setting_section: String,
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
    match system_setting::list() {
        Ok(settings) => web::Json(settings),
        Err(_) => web::Json(vec![]),
    }
}

async fn post(body: web::Json<SystemSetting>) -> impl Responder {
    let setting = body.into_inner();
    match system_setting::update(&setting.setting_section, 
        &setting.setting_key, 
        &setting.setting_value) 
    {
        Ok(_) => web::Json(Status { status: "Ok".to_string() }),
        Err(_) => web::Json(Status { status: "Error".to_string() }),
    }
}
