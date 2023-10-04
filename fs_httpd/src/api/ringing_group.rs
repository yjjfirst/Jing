use actix_web::{web, Responder};
use fs_lib::ringgroup;

pub fn ringing_group_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("")
                .route(web::get().to(index)))
        .service(
            web::resource("/ringing-groups/{id}")
                .route(web::get().to(get))
                .route(web::post().to(update)))
        .service(
            web::resource("/ringing-groups/{id}/members")
                .route(web::get().to(members)));


}

async fn index() -> impl Responder {
    let groups = ringgroup::all_ringgroup().unwrap();
    web::Json(groups)
}

async fn get(info: web::Path<i32>) -> impl Responder {
    let id = info.into_inner();
    let group = ringgroup::get_ringgroup(id).unwrap();
    web::Json(group)
}

async fn members(info: web::Path<i32>) -> impl Responder {
    let id = info.into_inner();
    let members = ringgroup::members(id).unwrap();
    web::Json(members)
}

async fn update(info: web::Json<ringgroup::models::Ringgroup>) -> impl Responder {

    web::Json("ok")
}
