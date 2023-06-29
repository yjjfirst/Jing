mod fs;
mod api;
use actix_web::{web, App, HttpServer};

use api::ringing_group::{ringing_group_config};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
            .service(fs::fs_post)
            .service(web::scope("/api").configure(ringing_group_config))
    })
        .bind("127.0.0.1:9090")?
        .bind("45.76.77.24:9090")?
        .run()
        .await
}
