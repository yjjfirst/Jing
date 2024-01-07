use std::ops::Deref;
use actix_web::{web, Responder};

use fs_lib::ringgroup;
use fs_lib::user;
use super::Status;

pub fn ringing_group_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("")
                .route(web::get().to(index)))
        .service(
            web::resource("/{id}")
                .route(web::get().to(get))
                .route(web::post().to(update)))
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
    let (domain,id) = path.into_inner();
    let group = ringgroup::get(id).unwrap();
    let members = ringgroup::members(id).unwrap();
    let users = user::users_within(domain).unwrap();

    web::Json((group,
               members,
               users.iter().map(|u|{
                   u.user_id.clone()
               }).collect::<Vec<String>>()
    ))
}

async fn members(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_, id) = path.into_inner();
    let members = ringgroup::members(id).unwrap();
    web::Json(members)
}

async fn update(group: web::Json<(ringgroup::models::Ringgroup, Vec<String>, Vec<String>)>) -> impl Responder {
    let (group, members, _) =  group.deref();
    let members_exist = ringgroup::members(group.id).unwrap();

    for m in members_exist {
        let user = user::get_user(user::ByField::UserId(m)).unwrap();
        ringgroup::del_ringgroup_member(group.id, user.id).unwrap();
    }

    for m in members {
        let user = user::get_user(user::ByField::UserId(m.to_string())).unwrap();
        ringgroup::add_ringgroup_member(group.id, user.id).unwrap();
    }

    ringgroup::update(group).unwrap();
    web::Json(Status {status: "Ok".to_string()})
}
