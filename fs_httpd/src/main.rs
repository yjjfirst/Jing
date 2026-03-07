mod fs;
mod api;
mod cdr;
use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::{from_fn, Next},
    Error, Result
};
use api::{api_config};
use fs_lib::portal_token::is_expired;

async fn cookie_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let cookie = req.cookie("token");

    if !req.path().starts_with("/api") {
            return next.call(req).await;
    }

    match cookie {
        Some(c) => {
            if !is_expired(c.value()) {
                return next.call(req).await;
            }

            if req.path() == "/api/login" {
                return next.call(req).await;
            }

            return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
        },
        None => {
            if req.path() == "/api/login" {
                return next.call(req).await;
            }

            return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
        }
    };

}

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("html/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        let cors = Cors::permissive();
        App::new()
            .app_data(web::FormConfig::default().limit(327_680))
            .wrap(cors)
            .wrap(from_fn(cookie_middleware))
            .service(Files::new("/admin", "./html").index_file("index.html"))
            .service(fs::fs_post)
            .service(cdr::cdr_post)
            .service(web::scope("/api").configure(api_config))
            .default_service(web::route().to(index))
    })
        .bind("0.0.0.0:9090")?
        .run()
        .await
}
