use actix_web::{web, Responder};
use fs_lib::ringgroup;

pub fn ringing_group_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/ringing-groups")
                .route(web::get().to(ringing_groups)))
        .service(
            web::resource("/ringing-groups/{id}")
                .route(web::get().to(ringing_group)))
        .service(
            web::resource("/ringing-groups/{id}/members")
                .route(web::get().to(ringing_group_members)));

}

async fn ringing_groups() ->  impl Responder {
    let groups = ringgroup::all_ringgroup().unwrap();
    web::Json(groups)
}

async fn ringing_group(info: web::Path<i32>) -> impl Responder {
    let id = info.into_inner();
    let group = ringgroup::get_ringgroup(id).unwrap();
    web::Json(group)
}

async fn ringing_group_members(info: web::Path<i32>) -> impl Responder {
    let id = info.into_inner();
    let members = ringgroup::members(id).unwrap();
    web::Json(members)
}
