use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use actix_web::{web, Responder};

use fs_lib::profile;
use fs_lib::profile::models::ProfileParam;

#[derive(Serialize, Deserialize)]
pub struct Profile {
    id: i32,
    name: String,
    params: HashMap<String, ProfileParam>
}

pub fn profile_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(index)))
        .service(
            web::resource("/{id}")
                .route(web::get().to(get))
                .route(web::post().to(post))
        );
}

async fn index(_path: web::Path<i32>) -> impl Responder {
    let profiles = profile::list().unwrap();

    web::Json(profiles.iter().map(|p|{
        Profile {
            id: p.id,
            name: p.name.clone(),
            params: HashMap::new()
        }
    }).collect::<Vec<Profile>>())
}

async fn get(path: web::Path<(i32,i32)>) -> impl Responder {
    let (_, id) = path.into_inner();
    let profile = profile::get_profile(id).unwrap();
    let params = profile::get_profile_params(profile.id).unwrap();

    web::Json(Profile {
        id: profile.id,
        name: profile.name.clone(),
        params: params
            .into_iter()
            .map(|p| {
                (p.name.clone(), p.clone())
            })
            .collect::<HashMap<String, ProfileParam>>()
    })
}

async fn post(path: web::Path<(i32,i32)>, prof: web::Json<Profile>) -> impl Responder {
    let (_, id) = path.into_inner();
    let params = prof.into_inner().params;
    let profile = profile::get_profile(id).unwrap();

    for (name, param) in params {
        profile::set_profile_param(param.id, &name, &param.value).unwrap();
    }

    web::Json(profile)
}