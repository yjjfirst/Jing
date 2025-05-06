use std::ops::Deref;
use actix_web::{web, Responder};
use super::Status;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use fs_lib::ivr;
use fs_lib::ivr::ivr_attrs::{IvrAttr};

#[derive(Serialize, Deserialize)]
struct Ivr {
    pub id: i32,
    pub exten: String,
    pub name: String,
    pub domain_id: i32,
    pub attrs: HashMap<String, IvrAttr>,
    pub entries: Vec<ivr::ivr_entry::IvrEntry>
}


pub fn ivr_config(cfg: &mut web::ServiceConfig) {
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
    let ivrs = ivr::list().unwrap();

    web::Json(ivrs.into_iter().map(|i|{
        Ivr {
            id: i.id,
            domain_id: i.domain_id,
            name: i.name.clone(),
            exten: i.exten.clone(),
            attrs: HashMap::new(),
            entries: vec![]
        }
    }).collect::<Vec<Ivr>>())
}


async fn get(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_, id) = path.into_inner();
    let ivr = ivr::get(id).unwrap();
    let attrs = ivr::ivr_attrs::list(id).unwrap();
    let entries = ivr::ivr_entry::list(id).unwrap();

    let attr_hash  = attrs.iter().map(|a|{
        (a.name.clone(), a.clone())
    }).collect::<HashMap<String, IvrAttr>>();

    web::Json(Ivr {
        id: ivr.id,
        name: ivr.name,
        exten: ivr.exten,
        domain_id: ivr.domain_id,
        attrs: attr_hash,
        entries: entries
    })
}

async fn post(ivr: web::Json<Ivr>) -> impl Responder {
    let ivr = ivr.deref();

    ivr::update(ivr::Ivr {
        id: ivr.id,
        domain_id: ivr.domain_id,
        name: ivr.name.clone(),
        exten: ivr.exten.clone()
    }).unwrap();

    for attr in &ivr.attrs {
        let attr = attr.1;
        if attr.id == 0 {
            ivr::ivr_attrs::add_attr(
                ivr.id,
                attr.name.clone(),
                attr.value.clone()).unwrap();
        }
        else {
            ivr::ivr_attrs::update(&attr).unwrap();
        }
    }

    for entry in &ivr.entries {
        if entry.id == 0 {
            ivr::ivr_entry::add_entry(
                ivr.domain_id,
                ivr.id,
                entry.digits.clone(),
                entry.dest_exten.clone()
            ).unwrap();
        }
        else {
            if entry.digits.len() != 0 {
                ivr::ivr_entry::update(&entry).unwrap();
            } else {
                ivr::ivr_entry::del_entry(entry.id).unwrap();
            }
        }
    }

    web::Json(Status {status: "Ok".to_string()})
}

async fn delete(_path: web::Path<(i32, i32)>) -> impl Responder {
    web::Json(Status {status: "Ok".to_string()})
}
