use std::ops::Deref;
use actix_web::{web, Responder};
use serde::{Serialize, Deserialize};

use fs_lib::acl::list as acl_list;
use fs_lib::acl::node as acl_node;

#[derive(Serialize, Deserialize)]
pub struct AclList {
    pub id: i32,
    pub acl_name: String,
    pub acl_default: String,
    pub nodes: Vec<AclNode>,
}

#[derive(Serialize, Deserialize)]
pub struct AclNode {
    pub id: i32,
    pub list_id: i32,
    pub node_type: String,
    pub cidr: String,
}

impl AclList {
    pub fn new() -> Self {
        AclList {
            id: 0,
            acl_name: "".to_string(),
            acl_default: "".to_string(),
            nodes: vec![]
        }
    }
}

pub fn acl_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("")
                .route(web::get().to(index)))
        .service(
            web::resource("/node/{id}")
                .route(web::post().to(node_post)))
        .service(
            web::resource("/{id}")
                .route(web::get().to(get))
                .route(web::post().to(post))
                .route(web::delete().to(delete)));
}

async fn index(_path: web::Path<i32>) -> impl Responder {
    let lists = acl_list::list().unwrap_or_default();

    web::Json(lists.iter().map(|l| {
        AclList {
            id: l.id,
            acl_name: l.acl_name.clone(),
            acl_default: l.acl_default.clone(),
            nodes: vec![],
        }
    }).collect::<Vec<AclList>>())
}

async fn get(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_domain, id) = path.into_inner();

    if id == 0 {
        return web::Json(AclList::new());
    }

    match acl_list::get(id) {
        Ok(list) => {
            let nodes = acl_node::list_by(Some(id)).unwrap_or_default();
            let node_dtos: Vec<AclNode> = nodes.into_iter().map(|n| {
                AclNode {
                    id: n.id,
                    list_id: n.list_id,
                    node_type: n.node_type,
                    cidr: n.cidr,
                }
            }).collect();

            web::Json(AclList {
                id: list.id,
                acl_name: list.acl_name,
                acl_default: list.acl_default,
                nodes: node_dtos,
            })
        }
        Err(_) => {
            web::Json(AclList::new())
        }
    }
}

async fn post(a: web::Json<AclList>) -> impl Responder {
    let acl = a.deref();
    if acl.id == 0 {
        acl_list::add(&acl.acl_name, &acl.acl_default).unwrap();
    } else {
        acl_list::edit(acl.id, &acl.acl_name, &acl.acl_default).unwrap();
    }
    web::Json(super::Status { status: "Ok".to_string() })
}

async fn delete(_path: web::Path<(i32, i32)>) -> impl Responder {
    web::Json(super::Status { status: "Ok".to_string() })
}

async fn node_post() -> impl Responder {
    web::Json(super::Status { status: "Ok".to_string() })
}
