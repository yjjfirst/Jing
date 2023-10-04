mod fs;
mod api;
use actix_web::{web, App, HttpServer};
use actix_cors::Cors;

use api::{api_config};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(fs::fs_post)
            .service(web::scope("/api").configure(api_config))
    })
        .bind("127.0.0.1:9090")?
        .bind("45.76.77.24:9090")?
        .run()
        .await
}
