use std::ops::Deref;
use std::collections::{HashMap};
use actix_web::{web, Responder};
use serde::{Serialize, Deserialize};
use super::Status;

use jlib::gateway;
use jlib::gateway::models;
use jlib::gateway::gateway_param;
use jlib::gateway::gateway_param::GatewayParam;
use jlib::gateway::gateway_param_help::{GatewayParamHelp, list as param_helps_list}; 

#[derive(Serialize, Deserialize)]
pub struct Gateway {
    id: i32,
    gateway_name: String,
    profile_id: i32,
    params: HashMap<String, GatewayParam>,
    param_helps: Vec<GatewayParamHelp>
}

pub fn gateway_config(cfg: &mut web::ServiceConfig) {
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

async fn index(_path: web::Path<i32>) -> impl Responder {
    let gws = gateway::list().unwrap();

    web::Json(gws.iter().map(|g|{
        Gateway {
            id: g.id,
            gateway_name: g.gateway_name.clone(),
            profile_id: g.profile_id,
            params: HashMap::new(),
            param_helps: vec![]
        }
    }).collect::<Vec<Gateway>>())
}

async fn get(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_,id)= path.into_inner();
    if id != 0 {
        let params = gateway::get_params(id).unwrap();
        let g = gateway::get(id).unwrap();
        let gateway = Gateway {
            id: g.id,
            gateway_name: g.gateway_name.clone(),
            profile_id: g.profile_id,
            params: params.iter().map(|p| {
                (p.name.clone(), p.clone())
            }).collect::<HashMap<String, GatewayParam>>(),
            param_helps: param_helps_list().unwrap()
        };
        web::Json(gateway)
    } else {
        let params = gateway_param::default_params();
        let params_hash = params.into_iter().map(|p|{
            (p.0.to_string(), gateway_param::GatewayParam {
                id: 0,
                gateway_id: 0,
                name: p.0.to_string(),
                value: p.1.to_string(),
            })
        }).collect();

        let gateway = Gateway {
            id: 0,
            profile_id: 2,
            gateway_name: "".to_string(),
            params: params_hash,
            param_helps: param_helps_list().unwrap()
        };
        web::Json(gateway)
    }
}

async fn post(g: web::Json<Gateway>) -> impl Responder {
    let gateway = g.deref();

    let model_gateway = models::Gateway {
            id: gateway.id,
            gateway_name: gateway.gateway_name.clone(),
            profile_id: gateway.profile_id
    };

    if gateway.id != 0 {
        gateway::update(&model_gateway).unwrap();
        update_params(gateway, gateway.id);
    } else {
        let id = gateway::add(gateway.profile_id,
                              gateway.gateway_name.clone(), HashMap::new()).unwrap();
        update_params(gateway, id);
    }

    web::Json(Status {status: "Ok".to_string()})

}

async fn delete(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_, id) = path.into_inner();

    gateway::del(id).unwrap();
    web::Json(Status {status: "Ok".to_string()})
}

fn update_params(gateway: &Gateway, gateway_id: i32) {
    for p in &gateway.params {
        let param = p.1;
        if param.id == 0 {
            gateway_param::add(gateway_id, param.name.clone(), param.value.clone()).unwrap();
        } else {
            let param = GatewayParam {
                gateway_id: gateway_id,
                id: param.id,
                name: param.name.clone(),
                value: param.value.clone()
            };
            gateway_param::update(&param).unwrap();
        }
    }
}
