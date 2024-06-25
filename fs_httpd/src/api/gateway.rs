use std::ops::Deref;
use actix_web::{web, Responder};
use serde::{Serialize, Deserialize};
use super::Status;

use fs_lib::gateway;
pub fn gateway_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("")
                .route(web::get().to(index)))
        .service(
            web::resource("/{id}")
                .route(web::get().to(get))
                .route(web::post().to(post)));
}

async fn index(_path: web::Path<i32>) -> impl Responder {
    let gws = gateway::list().unwrap();
    web::Json(gws)
}

async fn get(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_,id)= path.into_inner();

    let gateway = gateway::get(id).unwrap();

    web::Json(gateway)
}

async fn post(g: web::Json<gateway::models::Gateway>) -> impl Responder {
    let gateway = g.deref();

    gateway::update(gateway).unwrap();
    web::Json(Status {status: "Ok".to_string()})
}
