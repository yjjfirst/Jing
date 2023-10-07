use actix_web::{web, Responder};
use fs_lib::ringgroup;
use fs_lib::domain;

pub fn ringing_group_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("")
                .route(web::get().to(index)))
        .service(
            web::resource("/{id}")
                .route(web::get().to(get))
                .route(web::post().to(update)))
        .service(
            web::resource("/{id}/members")
                .route(web::get().to(members)));


}

async fn index(path: web::Path<String>) -> impl Responder {
    let domain = path.into_inner();
    let domain = domain::get_domain_by_name(domain).unwrap();

    let groups = ringgroup::groups_in_domain(domain.id);
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
