use std::ops::Deref;
use actix_web::{web, Responder, cookie::Cookie, HttpResponse};
use serde::Deserialize;

use super::Status;
use fs_lib::portal_user::{authorize};

#[derive(Deserialize)]
pub struct Credential {
    pub username: String,
    pub password: String
}

pub fn login_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("verify")
                .route(web::get().to(verify)))
        .service(
            web::resource("")
                .route(web::post().to(login))
        );
}

async fn verify() -> impl Responder {
    web::Json(Status {status: "Ok".to_string()})
}

async fn login(c: web::Json<Credential>) -> impl Responder {
    let Credential {username, password} = c.deref();
    let token = authorize(username, password).unwrap();

    if token.len() == 0 {
        return HttpResponse::Unauthorized()
            .body("Unauthorized.")
    }

    let cookie = Cookie::build("token", &token)
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .body("Login succeed.")
}
