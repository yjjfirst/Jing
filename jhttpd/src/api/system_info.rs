use sys_info::{disk_info};
use actix_web::{web, Responder};
use serde_json::json;

pub fn system_info_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("disk")
                .route(web::get().to(get_disk_info))
        );
}

async fn get_disk_info() -> impl Responder {

    let disk_info = disk_info().unwrap();
    let used = ( disk_info.total - disk_info.free ) / 1000000;
    let free_space = disk_info.free / 1000000;

    println!("Disk info: {:?}", disk_info);
    web::Json(json!({"used": used, "free": free_space}))
}