use std::ops::Deref;
use actix_web::{web, Responder};
use super::Status;
use jlib::route::inbound;
use jlib::route::inbound_models::InboundRoute;

pub fn inbound_config(cfg: &mut web::ServiceConfig) {
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
async fn post(r: web::Json<InboundRoute>) -> impl Responder {
    let route = r.deref();
    if route.id != 0 {
        inbound::update(route).unwrap();
    } else {
        inbound::add(&r.context, &r.condition, &r.dest_extension).unwrap();
    }

    web::Json(Status {status: "Ok".to_string()})
}
async fn get(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_,id) = path.into_inner();
    let inbound = inbound::get(id).unwrap_or(InboundRoute {
        id: 0,
        context: "".to_string(),
        condition: "".to_string(),
        dest_extension: "".to_string()
    });

    web::Json(inbound)
}

async fn delete(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_, id) = path.into_inner();
    inbound::del(id).unwrap();

    web::Json(Status {status: "Ok".to_string()})
}

async fn index(_path: web::Path<i32>) -> impl Responder {
    let routes = inbound::list().unwrap();
    web::Json(routes)
}
