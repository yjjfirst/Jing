use std::ops::Deref;
use actix_web::{web, Responder};
use super::Status;

use fs_lib::ivr;

pub fn ivr_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("")
                .route(web::get().to(index)))
        .service(
            web::resource("/{id}")
                .route(web::get().to(get))
                .route(web::post().to(post))
                .route(web::delete().to(delete))
        );
}

async fn index(_path: web::Path<i32>) -> impl Responder {
    let ivrs = ivr::list().unwrap();

    web::Json(ivrs)
}

async fn get(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_, id) = path.into_inner();
    let ivr = ivr::get(id).unwrap();

    web::Json(ivr)
}

async fn post(ivr: web::Json<ivr::Ivr>) -> impl Responder {
    let ivr = ivr.deref();

//    ivr::update(ivr).unwrap();

    web::Json(Status {status: "Ok".to_string()})
}

async fn delete(path: web::Path<(i32, i32)>) -> impl Responder {
    web::Json(Status {status: "Ok".to_string()})
}
