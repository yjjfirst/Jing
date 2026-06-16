use actix_web::{web, Responder};
use serde::{Serialize, Deserialize};

use fs_lib::db_connect;
use fs_lib::schema::{acl_lists, acl_nodes};

#[derive(Serialize, Deserialize)]
pub struct AclList {
    pub id: i32,
    pub acl_name: String,
    pub acl_default: String,
}

#[derive(Serialize, Deserialize)]
pub struct AclNode {
    pub id: i32,
    pub list_id: Option<i32>,
    pub node_type: String,
    pub cidr: String,
}

pub fn acl_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("")
                .route(web::get().to(index)))
        .service(
            web::resource("/{id}")
                .route(web::get().to(get))
                .route(web::post().to(post))
                .route(web::delete().to(delete)));
}

async fn index(path: web::Path<i32>) -> impl Responder {
    web::Json(super::Status { status: "Ok".to_string() })
}

async fn get(path: web::Path<(i32, i32)>) -> impl Responder {
    web::Json(super::Status { status: "Ok".to_string() })
}

async fn post(_p: web::Json<AclList>) -> impl Responder {
    web::Json(super::Status { status: "Ok".to_string() })
}

async fn delete(path: web::Path<(i32, i32)>) -> impl Responder {
    web::Json(super::Status { status: "Ok".to_string() })
}
