use actix_web::{web, Responder};
use fs_lib::user;

pub fn extension_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("")
                .route(web::get().to(index)));
}

async fn index(path: web::Path<i32>) -> impl Responder {
    let domain = path.into_inner();
    let users = user::users_within(domain).unwrap();
    web::Json(users)
}
