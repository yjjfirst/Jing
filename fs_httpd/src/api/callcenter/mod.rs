pub mod queue;

use std::ops::Deref;
use actix_web::{web, Responder};
use super::Status;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub fn cc_config(cfg: & mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/queue")
                .route(web::get().to(queue::index)))
        .service(
            web::resource("/queue/{id}")
                .route(web::get().to(queue::get))
                .route(web::post().to(queue::post))
                .route(web::delete().to(queue::delete))
        );
}
