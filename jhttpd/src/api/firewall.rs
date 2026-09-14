use super::Status;

use jlib::firewall;
use actix_web::{error::ErrorInternalServerError, web, Error, Responder};

pub fn firewall_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
    web::resource("")
                .route(web::get().to(index)))
        .service(
            web::resource("/{id}")
                .route(web::post().to(post))
    );
}

async fn index(_path: web::Path<i32>) -> Result<web::Json<Vec<firewall::FirewallRule>>, Error> {
    firewall::list()
        .map(web::Json)
        .map_err(|err| ErrorInternalServerError(err.to_string()))
}

async fn post(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_, id) = path.into_inner();

    firewall::toggle(id).unwrap();
    web::Json(Status { status: "Ok".to_string() })
}