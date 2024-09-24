mod fs;
mod api;
mod cdr;
use actix_web::{web, App, HttpServer};
use actix_cors::Cors;

use api::{api_config};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        let cors = Cors::permissive();
        App::new()
            .app_data(web::FormConfig::default().limit(327_680))
            .wrap(cors)
            .service(fs::fs_post)
            .service(cdr::cdr_post)
            .service(web::scope("/api").configure(api_config))
    })
        .bind("0.0.0.0:9090")?
        .run()
        .await
}
