use actix_web::{web, Responder};
use super::Status;

pub fn outbound_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("")
                .route(web::get().to(index)));
}

async fn index(_path: web::Path<i32>) -> impl Responder {
    web::Json(Status {status: "Ok".to_string()})
}
