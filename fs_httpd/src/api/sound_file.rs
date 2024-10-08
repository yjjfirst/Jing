use std::ops::Deref;
use actix_web::{web, Responder};
use fs_lib::sound_file;
use fs_lib::sound_file::models::{SoundFile};
use super::Status;

pub fn sound_file_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("")
                .route(web::get().to(index)))
        .service(
            web::resource("/{id}")
                .route(web::get().to(get))
                .route(web::post().to(post))
        );
}

async fn index() -> impl Responder {
    let files = sound_file::list().unwrap();
    web::Json(files)
}

async fn get(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_,id) =path.into_inner();
    let s = sound_file::get(id).unwrap_or(SoundFile {
        id: 0,
        domain_id: 0,
        name:"".to_string(),
        description: Some("".to_string())
    });

    web::Json(s)
}

async fn post(s: web::Json<SoundFile>) -> impl Responder {
    let f = s.deref();
    if f.id != 0 {
        sound_file::update(f.clone());
    } else {
    }

    web::Json(Status {status: "Ok".to_string()})
}
