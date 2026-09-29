use actix_web::{web,Responder};
use jlib::fs::{show_channels};

pub fn channel_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("")
                .route(web::get().to(index)));
}

async fn index() -> impl Responder {
    let channels = show_channels();
    web::Json(channels)
}
