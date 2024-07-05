use std::ops::Deref;
use actix_web::{web, Responder};
use super::Status;
use fs_lib::route::outbound;
use fs_lib::route::outbound_models::OutboundRoute;

pub fn outbound_config(cfg: &mut web::ServiceConfig) {
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

async fn delete(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_, id) = path.into_inner();
    outbound::del(id).unwrap();
    web::Json(Status {status: "Ok".to_string()})
}

async fn index(_path: web::Path<i32>) -> impl Responder {
    let routes = outbound::list().unwrap();
    web::Json(routes)
}

async fn get(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_, id) = path.into_inner();
    let out = outbound::get(id).unwrap_or( OutboundRoute {
        id: 0, gateway_id: 0, condition: "".to_string(), priority: 100
    });

    web::Json(out)
}

async fn post(r: web::Json<OutboundRoute>) -> impl Responder {
    let route = r.deref();
    if route.id != 0 {
        outbound::update(route);
    } else {
        outbound::add(r.gateway_id, r.priority, &r.condition).unwrap();
    }

    web::Json(Status {status: "Ok".to_string()})
}
