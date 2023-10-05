use actix_web::{web,Responder};
use fs_lib::domain;

pub fn domain_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("")
                .route(web::get().to(index)));
}

async fn index() -> impl Responder {
    let domains = domain::list_domains().unwrap();
    web::Json(domains)
}
