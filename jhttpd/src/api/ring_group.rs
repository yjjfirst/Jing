use std::ops::Deref;
use actix_web::{web, Responder};
use serde::{Serialize, Deserialize};

use fs_lib::ringgroup;
use fs_lib::user;
use super::Status;

#[derive(Serialize, Deserialize)]
pub struct Ringgroup {
    pub id: i32,
    pub name: String,
    pub group_id: String,
    pub domain_id: i32,
    pub description: Option<String>,
    pub ring_time: i32,
    pub ring_strategy: String,
    pub members: Vec<String>
}

impl Ringgroup {
    pub fn new(domain_id: i32) -> Ringgroup {
        Ringgroup {
            id: 0,
            name: "".to_string(),
            group_id: "".to_string(),
            domain_id,
            description: Some("".to_string()),
            ring_time: 20,
            ring_strategy: "sequential".to_string(),
            members: vec![]
        }
    }
}

impl From<ringgroup::models::Ringgroup> for Ringgroup {
    fn from(g: ringgroup::models::Ringgroup) -> Self {
        let ringgroup::models::Ringgroup {
            id, name, group_id, domain_id, description, ring_time, ring_strategy
        } = g;

        Ringgroup {
            id, name, group_id, domain_id, description, ring_time, ring_strategy,
            members: vec![]
        }
    }
}

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
    web::Json(groups
              .into_iter()
              .map(|g|{Ringgroup::from(g)})
              .collect::<Vec<Ringgroup>>()
    )
}

async fn get(path: web::Path<(i32, i32)>) -> impl Responder {
    let (domain_id,id) = path.into_inner();

    if id != 0 {
        let g = ringgroup::get(id).unwrap();
        let members = ringgroup::members(id).unwrap();
        let mut group = Ringgroup::from(g);
        group.members = members;
        web::Json(group)
    } else {
        web::Json(
            Ringgroup::new(domain_id)
        )
    }
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

async fn post(group: web::Json<Ringgroup>) -> impl Responder {
    let group =  group.deref();
    let members_exist = ringgroup::members(group.id).unwrap();


    if group.id != 0 {
        for m in members_exist {
            let user = user::get_user(user::ByField::UserId(m)).unwrap();
            ringgroup::del_ringgroup_member(group.id, user.id).unwrap();
        }

        for m in group.members.clone() {
            let user = user::get_user(user::ByField::UserId(m.to_string())).unwrap();
            ringgroup::add_ringgroup_member(group.id, user.id).unwrap();
        }

        ringgroup::update(&ringgroup::models::Ringgroup {
            id: group.id,
            group_id: group.group_id.clone(),
            name: group.name.clone(),
            description: group.description.clone(),
            domain_id: group.domain_id,
            ring_time: group.ring_time,
            ring_strategy:  group.ring_strategy.clone()
        }).unwrap();
    } else {
        let inserted = ringgroup::add_ringgroup(group.domain_id,
                                 group.name.clone(),
                                 group.group_id.clone(),
                                 group.description.clone(),
                                 Some(group.ring_time),
                                 Some(group.ring_strategy.clone())).unwrap();
        for m in group.members.clone() {
            let user = user::get_user(user::ByField::UserId(m.to_string())).unwrap();
            ringgroup::add_ringgroup_member(inserted.id, user.id).unwrap();
        }
    }
    web::Json(Status {status: "Ok".to_string()})
}
