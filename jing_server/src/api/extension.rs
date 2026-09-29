use actix_web::{web,Responder};
use jlib::extension;

pub fn extension_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("")
                .route(web::get().to(index)));
}

async fn index(path: web::Path<i32>) -> impl Responder {
    let domain_id = path.into_inner();
    let extensions = extension::list(domain_id).unwrap();

    web::Json(extensions)
}
