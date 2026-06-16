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
            .route(web::get().to(index)));
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
