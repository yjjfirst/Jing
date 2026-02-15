use std::ops::Deref;
use std::collections::HashMap;
use actix_web::{web, Responder};
use serde::{Serialize, Deserialize};

use fs_lib::callcenter::tier;
use fs_lib::callcenter::agent::{get};
use fs_lib::user::{get_user, ByField};
use super::agent::{Agent};

use super::Status;

#[derive(Deserialize)]
pub struct Info {
    queue_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Tier {
    pub id: i32,
    pub queue_id: i32,
    pub level: i32,
    pub position: i32,
    pub agent: Agent
}

pub async fn index(info: web::Query<Info>) -> impl Responder {
    let queue_id = info.queue_id;
    let tiers = tier::list(queue_id).unwrap();

    web::Json(tiers.iter().map(|t|{
        let a = get(t.agent_id).unwrap();
        let user =  get_user(ByField::Id(a.user_id)).unwrap();
        Tier {
            id: t.id,
            queue_id: t.queue_id,
            level: t.level,
            position: t.position,
            agent: Agent {
                id: a.id,
                domain_id: a.domain_id,
                user_id: a.user_id,
                name: a.name.clone(),
                contact: user.user_id.clone(),
                leg_timeout: a.leg_timeout,
                params: HashMap::new()
            }
        }
    }).collect::<Vec<Tier>>())
}

pub async fn post(tier: web::Json<Tier>) -> impl Responder {
    let tier = tier.deref();

    if tier.id != 0 {
        tier::update(tier::Tier {
            id: tier.id,
            agent_id: tier.agent.id,
            queue_id: tier.queue_id,
            level: tier.level,
            position: tier.position
        }).unwrap();
    } else {
        tier::add(tier.agent.id, tier.queue_id, tier.level, tier.position).unwrap();
    }

    web::Json(Status {status: "Ok".to_string()})
}

pub async fn delete(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_, id) = path.into_inner();
    tier::del(id).unwrap();
    web::Json(Status {status: "Ok".to_string()})
}
