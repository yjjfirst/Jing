use actix_web::{web, Responder, HttpResponse, HttpRequest};

use fs_lib::portal_token::{revoke};

pub fn logout_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("")
                .route(web::post().to(logout))
        );
}

async fn logout(req: HttpRequest) -> impl Responder {
    let c = req.cookie("token").unwrap();

    revoke(c.value()).unwrap();
    HttpResponse::Ok()
        .body("Logout succeed.")
}
