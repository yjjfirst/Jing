mod fs;
use fs_lib::ringgroup;

use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize};


fn ringing_group_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/ringing-groups")
            .route(web::get().to(ringing_groups))
    );
}

async fn ringing_groups() ->  impl Responder {
    let groups = ringgroup::all_ringgroup().unwrap();
    web::Json(groups)
}

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
