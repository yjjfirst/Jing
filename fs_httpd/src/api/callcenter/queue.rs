use std::ops::Deref;
use actix_web::{web, Responder};
use super::Status;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use fs_lib::queue;
use fs_lib::queue::queue_param;
use fs_lib::queue::queue_param::QueueParam;

#[derive(Serialize, Deserialize)]
pub struct Queue {
    pub id: i32,
    pub exten: String,
    pub name: String,
    pub domain_id: i32,
    pub params: HashMap<String, QueueParam>,
}

pub async fn index(path: web::Path<i32>) -> impl Responder {
    let domain_id = path.into_inner();
    let queues = queue::queues_in(domain_id).unwrap();

    web::Json(queues.into_iter().map(|q|{
        Queue {
            id: q.id,
            exten: q.exten,
            name: q.name,
            domain_id: q.domain_id,
            params: HashMap::new()
        }
    }).collect::<Vec<Queue>>())
}

pub async fn get(path: web::Path<(i32, i32)>)-> impl Responder {
    let (domain_id, id) = path.into_inner();
    if id != 0 {
        let queue = queue::get(id).unwrap();
        let params = queue::params(id).unwrap();

        web::Json(Queue {
            id: queue.id,
            exten: queue.exten,
            name: queue.name,
            domain_id: queue.domain_id,
            params: params.into_iter().map(|p|{
                (p.name.clone(), p.clone())
            }).collect()
        })
    } else {
        let params = queue_param::default_params();
        let params_hash = params.into_iter().map(|p|{
            (p.0.to_string(), queue_param::QueueParam {
                id: 0,
                queue_id: 0,
                name: p.0.to_string(),
                value: p.1.to_string(),
            })
        }).collect();
        web::Json(Queue {
            id: 0,
            exten: "".to_string(),
            name: "".to_string(),
            domain_id: domain_id,
            params: params_hash
        })
    }
}

pub async fn post(queue: web::Json<Queue>) -> impl Responder {
    let queue = queue.deref();
    if queue.id != 0 {
        queue::update(queue::Queue {
            id: queue.id,
            domain_id: queue.domain_id,
            exten: queue.exten.clone(),
            name: queue.name.clone()
        }).unwrap();

        update_params(queue, queue.id);
    } else {
        let id = queue::add(queue.domain_id,
                   queue.exten.clone(),
                            queue.name.clone()).unwrap();
        update_params(queue, id);
    }

    web::Json(Status {status: "Ok".to_string()})
}

pub async fn delete(path: web::Path<(i32, i32)>) -> impl Responder {
    let (domain_id, id) = path.into_inner();
    queue::del(id).unwrap();

    web::Json(Status {status: "Ok".to_string()})
}

fn update_params(queue: &Queue, queue_id: i32) {
    for p in &queue.params {
        let param = p.1;
        if param.id == 0 {
            queue_param::add(queue_id, param.name.clone(), param.value.clone()).unwrap();
        } else {
            let queue = QueueParam {
                queue_id: queue_id,
                id: param.id,
                name: param.name.clone(),
                value: param.value.clone()
            };

            queue_param::update(&queue).unwrap()
        }
    }
}
