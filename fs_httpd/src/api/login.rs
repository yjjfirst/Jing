use std::ops::Deref;
use actix_web::{web, Responder, cookie::Cookie, HttpResponse, HttpRequest};
use serde::Deserialize;

use super::Status;
use fs_lib::portal_user::{authorize, get as get_portal_user};
use fs_lib::portal_token::{get};

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

async fn verify(req: HttpRequest) -> impl Responder {
    let c = req.cookie("token").unwrap();
    let token = get(c.value());
    let user = get_portal_user(token.portal_user_id).unwrap();

    web::Json(Status {status: user.username})
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
