use std::ops::Deref;
use actix_web::{web, Responder};
use super::Status;

use fs_lib::gateway;
use fs_lib::gateway::models::Gateway;
pub fn gateway_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("")
                .route(web::get().to(index)))
        .service(
            web::resource("/{id}")
                .route(web::get().to(get))
                .route(web::post().to(post))
                .route(web::delete().to(delete)));
}

async fn index(_path: web::Path<i32>) -> impl Responder {
    let gws = gateway::list().unwrap();
    web::Json(gws)
}

async fn get(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_,id)= path.into_inner();

    let gateway = gateway::get(id).unwrap_or(Gateway {
        id: 0,
        gateway_name: "".to_string(),
        profile_id: 2,
        proxy: "".to_string(),
        register: "".to_string(),
        username: Some( "".to_string()),
        password: Some("".to_string())
    });

    web::Json(gateway)
}

async fn post(g: web::Json<gateway::models::Gateway>) -> impl Responder {
    let gateway = g.deref();

    if gateway.id != 0 {
        gateway::update(gateway).unwrap();
    } else {
        gateway::add(gateway.profile_id,
                     gateway.gateway_name.clone(),
                     gateway.proxy.clone(),
                     gateway.register.clone(),
                     gateway.username.clone(),
                     gateway.password.clone()
        ).unwrap();
    }

    web::Json(Status {status: "Ok".to_string()})

}

async fn delete(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_, id) = path.into_inner();

    gateway::del(id).unwrap();
    web::Json(Status {status: "Ok".to_string()})
}
