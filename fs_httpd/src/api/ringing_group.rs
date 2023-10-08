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

async fn index(path: web::Path<i32>) -> impl Responder {
    let domain = path.into_inner();
    let groups = ringgroup::groups_in_domain(domain);
    web::Json(groups)
}

async fn get(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_,id) = path.into_inner();
    let group = ringgroup::get_ringgroup(id).unwrap();
    web::Json(group)
}

async fn members(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_, id) = path.into_inner();
    let members = ringgroup::members(id).unwrap();
    web::Json(members)
}

async fn update(group: web::Json<ringgroup::models::Ringgroup>) -> impl Responder {

    println!("{:?}", group);
    web::Json("ok")
}
