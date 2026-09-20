use sys_info::{disk_info, loadavg, mem_info};
use actix_web::{web, Responder};
use serde_json::json;

pub fn system_info_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("disk")
                .route(web::get().to(get_disk_info)))
        .service(
            web::resource("loading")
                .route(web::get().to(get_loading_info)))
        .service(
            web::resource("memory")
                .route(web::get().to(get_memory_info))
        );
}

async fn get_disk_info() -> impl Responder {

    let disk_info = disk_info().unwrap();
    let used = ( disk_info.total - disk_info.free ) / 1000000;
    let free_space = disk_info.free / 1000000;

    web::Json(json!({"used": used, "free": free_space}))
}

async fn get_loading_info() -> impl Responder {
    let loading = loadavg().unwrap();

    web::Json(json!({"one": loading.one}))
}

async fn get_memory_info() -> impl Responder {
    let mem = mem_info().unwrap();

    web::Json(json!({
        "free": mem.free,
        "used": mem.total - mem.free,
        "avail": mem.avail,
    }))
}