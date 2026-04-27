mod fs;
mod api;
mod cdr;

use actix_web::{web, App, HttpServer};
use rustls::{ServerConfig};
use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::{from_fn, Next},
    Error, Result
};
use std::fs::File;
use std::io::BufReader;

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
            if is_expired(c.value()) {
                if req.path() == "/api/login" {
                    return next.call(req).await;
                }

                return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
            }

            return next.call(req).await;
        },
        None => {
            if req.path() == "/api/login" {
                return next.call(req).await;
            }

            return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
        }
    };

}

async fn index() -> Result<NamedFile, Error> {
    Ok(NamedFile::open("html/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_rustls_config();

    HttpServer::new(||{
        let cors = Cors::permissive();
        App::new()
            .app_data(web::FormConfig::default().limit(327_680))
            .wrap(cors)
            .wrap(from_fn(cookie_middleware))
            .service(fs::fs_post)
            .service(cdr::cdr_post)
            .service(web::scope("/api").configure(api_config))
            .service(Files::new("/", "/var/www/pbx").index_file("index.html"))
            .default_service(web::route().to(index))
    })
        .bind_rustls_0_23("137.220.37.143:9090", config)?
        .bind("127.0.0.1:9090")?
        .run()
        .await
}


fn load_rustls_config() -> ServerConfig {
    // Example helper to load cert.pem and key.pem
    let cert_file = &mut BufReader::new(File::open("/etc/letsencrypt/live/pbx.telman.me/cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("/etc/letsencrypt/live/pbx.telman.me/privkey.pem").unwrap());

    let cert_chain = rustls_pemfile::certs(cert_file)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let mut keys = rustls_pemfile::pkcs8_private_keys(key_file)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, keys.remove(0).into())
        .expect("bad certificate/key")
}
