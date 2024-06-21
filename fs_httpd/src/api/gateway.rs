use actix_web::{web, Responder};
use serde::{Serialize, Deserialize};
use super::Status;

use fs_lib::gateway;
pub fn gateway_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("")
                .route(web::get().to(index)));
}
async fn index(path: web::Path<i32>) -> impl Responder {
    let gws = gateway::list().unwrap();
    web::Json(gws)
}
