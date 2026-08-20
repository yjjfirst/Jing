use std::ops::Deref;
use actix_web::{web, Responder};
use super::Status;

use jlib::conference::{Conference};
use jlib::conference;

pub fn conf_config(cfg: &mut web::ServiceConfig) {
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
    let confs = conference::list().unwrap();

    web::Json(confs)
}


async fn get(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_, id) = path.into_inner();

    let sound = conference::get(id).unwrap_or(Conference {
        id: 0,
        name: "".to_string(),
        exten: "".to_string(),
        domain_id: 0,
        conference_profile_id: 0,
        description: "".to_string()
    });

    web::Json(sound)
}

async fn post(conf: web::Json<conference::Conference>) -> impl Responder {
    let conf = conf.deref();
    if conf.id != 0 {
        conference::update(conf).unwrap();
    } else  {
        conference::add(
            conf.domain_id,
            conf.conference_profile_id,
            conf.exten.clone(),
            conf.name.clone(),
            conf.description.clone()
        ).unwrap();
    }

    web::Json(Status {status: "Ok".to_string()})
}

async fn delete(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_, id) = path.into_inner();

    conference::del(id).unwrap();
    web::Json(Status {status: "Ok".to_string()})
}
