use actix_web::{web, Responder};

use fs_lib::cdr;
pub fn cdr_config(cfg: &mut web::ServiceConfig) {
    cfg.
        service(
            web::resource("")
                .route(web::get().to(index)));
}

async fn index(_path: web::Path<i32>) -> impl Responder {
    let cdrs = cdr::list().unwrap();
    web::Json(cdrs)
}
