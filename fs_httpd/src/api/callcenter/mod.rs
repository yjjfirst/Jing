pub mod queue;
pub mod agent;
pub mod tier;

use actix_web::{web};
use super::Status;

pub fn cc_config(cfg: & mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/queue")
                .route(web::get().to(queue::index)))
        .service(
            web::resource("/queue/{id}")
                .route(web::get().to(queue::get))
                .route(web::post().to(queue::post))
                .route(web::delete().to(queue::delete)))
        .service(
            web::resource("agent")
                .route(web::get().to(agent::index)))
        .service(
            web::resource("agent/{id}")
                .route(web::get().to(agent::get))
                .route(web::post().to(agent::post))
                .route(web::delete().to(agent::delete)))
        .service(
            web::resource("tier")
                .route(web::get().to(tier::index)))
        .service(
            web::resource("tier/{id}")
                .route(web::post().to(tier::post))
                .route(web::delete().to(tier::delete)));
}
