use std::ops::Deref;
use actix_web::{web, Responder};

use fs_lib::ringgroup;
use fs_lib::user;
use super::Status;

pub fn ring_group_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("")
                .route(web::get().to(index)))
        .service(
            web::resource("/{id}")
                .route(web::get().to(get))
                .route(web::delete().to(delete))
                .route(web::post().to(post)))
        .service(
            web::resource("/{id}/members")
                .route(web::get().to(members)));
}

async fn index(path: web::Path<i32>) -> impl Responder {
    let domain = path.into_inner();
    let groups = ringgroup::groups_in_domain(domain);
    web::Json(groups)
}

async fn get(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_domain,id) = path.into_inner();
    let group = ringgroup::get(id).unwrap();
    let members = ringgroup::members(id).unwrap();

    web::Json((group,
               members))
}

async fn delete(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_,id) = path.into_inner();

    ringgroup::del_ringgroup(id).unwrap();
    web::Json(Status {status: "Ok".to_string()})
}

async fn members(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_, id) = path.into_inner();
    let members = ringgroup::members(id).unwrap();
    web::Json(members)
}

async fn post(group: web::Json<(ringgroup::models::Ringgroup, Vec<String>)>) -> impl Responder {
    let (group, members) =  group.deref();
    let members_exist = ringgroup::members(group.id).unwrap();


    if group.id != 0 {
        for m in members_exist {
            let user = user::get_user(user::ByField::UserId(m)).unwrap();
            ringgroup::del_ringgroup_member(group.id, user.id).unwrap();
        }

        for m in members {
            let user = user::get_user(user::ByField::UserId(m.to_string())).unwrap();
            ringgroup::add_ringgroup_member(group.id, user.id).unwrap();
        }
        ringgroup::update(group).unwrap();
    } else {
        let inserted = ringgroup::add_ringgroup(group.domain_id,
                                 group.name.clone(),
                                 group.group_id.clone(),
                                 group.description.clone(),
                                 Some(group.ring_time),
                                 Some(group.ring_strategy.clone())).unwrap();
        for m in members {
            let user = user::get_user(user::ByField::UserId(m.to_string())).unwrap();
            ringgroup::add_ringgroup_member(inserted.id, user.id).unwrap();
        }
    }
    web::Json(Status {status: "Ok".to_string()})
}
