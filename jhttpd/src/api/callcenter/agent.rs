use std::ops::Deref;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use actix_web::{web, Responder};

use fs_lib::callcenter::agent;
use fs_lib::callcenter::agent_param;
use fs_lib::callcenter::agent_param::AgentParam;
use fs_lib::user::{get_user, ByField};

use super::Status;

#[derive(Serialize, Deserialize, Debug)]
pub struct Agent {
    pub id: i32,
    pub domain_id: i32,
    pub user_id: i32,
    pub contact: String,
    pub name: String,
    pub leg_timeout: i32,
    pub params: HashMap<String, AgentParam>
}

pub async fn index(path: web::Path<i32>) -> impl Responder {
    let domain_id = path.into_inner();
    let agents = agent::list(domain_id).unwrap();

    web::Json(agents.into_iter().map(|a| {
        let user = get_user(ByField::Id(a.user_id)).unwrap();
        Agent {
            id: a.id,
            domain_id: a.domain_id,
            user_id: a.user_id,
            name: a.name.clone(),
            contact: user.user_id.clone(),
            leg_timeout: a.leg_timeout,
            params: HashMap::new()
        }
    }).collect::<Vec<Agent>>())
}

pub async fn get(path: web::Path<(i32, i32)>) -> impl Responder {
    let (domain_id, id) = path.into_inner();

    if id != 0 {
        let agent = agent::get(id).unwrap();
        let user = get_user(ByField::Id(agent.user_id)).unwrap();
        let params = agent::params(id).unwrap();

        web::Json(Agent {
            id: agent.id,
            domain_id: agent.domain_id,
            user_id: agent.user_id,
            contact: user.user_id.clone(),
            name: agent.name.clone(),
            leg_timeout: agent.leg_timeout,
            params: params.into_iter().map(|a| {
                (a.name.clone(), a.clone())
            }).collect()
        })
    } else {
        let params = agent_param::default_params();
        let params_hash = params.into_iter().map(|p|{
            (p.0.to_string(), agent_param::AgentParam {
                id: 0,
                agent_id: 0,
                name: p.0.to_string(),
                value: p.1.to_string()
            })
        }).collect();
        web::Json(Agent{
            id: 0,
            user_id: 0,
            domain_id : domain_id,
            name: "".to_string(),
            contact: "".to_string(),
            leg_timeout: 15,
            params: params_hash
        })
    }
}

pub async fn post(agent: web::Json<Agent>) -> impl Responder {
    let agent = agent.deref();
    let user = get_user(ByField::UserId(agent.contact.clone())).unwrap();
    if agent.id != 0 {
        agent::update(agent::Agent {
            id: agent.id,
            domain_id: agent.domain_id,
            user_id: user.id,
            name: agent.name.clone(),
            leg_timeout: agent.leg_timeout
        }).unwrap();
        update_params(&agent, agent.id);
    } else {
        let id = agent::add(agent.domain_id,
                   user.id,
                   agent.name.clone(),
                   agent.leg_timeout

        ).unwrap();
        update_params(&agent, id);
    }

    web::Json(Status {status: "Ok".to_string()})
}

fn update_params(agent: &Agent, agent_id: i32) {
    for p in &agent.params {
        let param = p.1;
        if param.id == 0 {
            agent_param::add(agent_id, param.name.clone(),param.value.clone()).unwrap();
        } else {
            let p = AgentParam {
                id: param.id,
                agent_id: agent_id,
                name: param.name.clone(),
                value: param.value.clone()
            };

            agent_param::update(&p).unwrap();
        }
    }
}

pub async fn delete(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_, id) = path.into_inner();
    agent::del(id).unwrap();

    web::Json(Status {status: "Ok".to_string()})
}
