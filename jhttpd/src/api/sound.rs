use std::ops::Deref;
use actix_web::{web, Responder};
use super::Status;
use serde::Serialize;

use fs_lib::sound;
use fs_lib::sound_file;
use fs_lib::sound_file::models::{SoundFile};
use fs_lib::sound::models::Sound;

#[derive(Serialize)]
struct ApiSound {
    pub sound: Sound,
    pub sound_file: SoundFile,
}

pub fn sound_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("")
                .route(web::get().to(index)))
        .service(
            web::resource("/{id}")
                .route(web::get().to(get))
                .route(web::post().to(post))
                .route(web::delete().to(delete))
        );
}

async fn index(_path: web::Path<i32>) -> impl Responder {
    let sounds = sound::list().unwrap();

    let res: Vec<ApiSound> = sounds
        .iter()
        .map(|s|{
            let f = sound_file::get(s.sound_file_id).unwrap();
            ApiSound {
                sound: s.clone(),
                sound_file: f.clone()
            }
        })
        .collect();

    web::Json(res)
}

async fn post(s: web::Json<sound::models::Sound>) -> impl Responder {
    let sound = s.deref();

    if sound.id != 0 {
        sound::update(sound).unwrap();
    } else {
        sound::add(sound.domain_id,
                   sound.sound_file_id,
                   sound.name.clone(),
                   sound.exten.clone()).unwrap();
    }

    web::Json(Status {status: "Ok".to_string()})
}

async fn get(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_, id) = path.into_inner();

    let sound = sound::get(id).unwrap_or(Sound {
        id: 0,
        name: "".to_string(),
        exten: "".to_string(),
        domain_id: 0,
        sound_file_id: 0,
    });

    web::Json(sound)
}

async fn delete(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_, id) = path.into_inner();

    sound::del(id).unwrap();
    web::Json(Status {status: "Ok".to_string()})
}
