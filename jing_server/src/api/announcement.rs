use std::ops::Deref;
use actix_web::{web, Responder};
use super::Status;
use serde::{Serialize, Deserialize};

use jing_lib::announcement;
use jing_lib::sound_file;
use jing_lib::sound_file::models::{SoundFile};
use jing_lib::announcement::models;

#[derive(Serialize, Deserialize)]
struct Announcement {
    pub id: i32,
    pub exten: String,
    pub name: String,
    pub domain_id: i32,
    pub sound_file: SoundFile,
}

pub fn announcement_config(cfg: &mut web::ServiceConfig) {
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
    let announcements = announcement::list().unwrap();

    let res: Vec<Announcement> = announcements
        .iter()
        .map(|a|{
            let f = sound_file::get(a.sound_file_id).unwrap();
            Announcement {
                id: a.id,
                exten: a.exten.clone(),
                name: a.name.clone(),
                domain_id: a.domain_id,
                sound_file: f.clone()
            }
        })
        .collect();

    web::Json(res)
}

async fn post(s: web::Json<Announcement>) -> impl Responder {
    let announcement = s.deref();

    if announcement.id != 0 {
        announcement::update(&models::Announcement {
            id: announcement.id,
            name: announcement.name.clone(),
            exten: announcement.exten.clone(),
            domain_id: announcement.domain_id,
            sound_file_id: announcement.sound_file.id,
        }).unwrap();
    } else {
        announcement::add(announcement.domain_id,
                   announcement.sound_file.id,
                   announcement.name.clone(),
                   announcement.exten.clone()).unwrap();
    }

    web::Json(Status {status: "Ok".to_string()})
}

async fn get(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_, id) = path.into_inner();

    let announcement = announcement::get(id).unwrap_or(models::Announcement {
        id: 0,
        name: "".to_string(),
        exten: "".to_string(),
        domain_id: 0,
        sound_file_id: 0,
    });

    web::Json(Announcement {
        id: announcement.id,
        exten: announcement.exten,
        name: announcement.name,
        domain_id: announcement.domain_id,
        sound_file: sound_file::get(announcement.sound_file_id).unwrap(),
    })
}

async fn delete(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_, id) = path.into_inner();

    announcement::del(id).unwrap();
    web::Json(Status {status: "Ok".to_string()})
}
