use std::collections::HashMap;
use actix_web::{web, Responder};
use fs_lib::{user};
use fs_lib::user::{ByField};
use fs_lib::user::models;
use fs_lib::user::user_param::UserParam;
use fs_lib::user::user_variable::UserVariable;
use serde::{Serialize, Deserialize};

use super::Status;
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub domain_id: i32,
    pub user_id: String,
    vars: HashMap<String, UserVariable>,
    params: HashMap<String, UserParam>,
}

pub fn user_config(cfg: &mut web::ServiceConfig) {
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

async fn post(user: web::Json<User>) -> impl Responder {
    let vars = &user.vars;
    let params = &user.params;

    if user.id != 0 {
        for (name, var) in vars.into_iter() {
            if var.id == 0 {
                UserVariable::add(user.id, name, &var.value).unwrap();
            } else {
                UserVariable::update(var.id, name, &var.value).unwrap();
            }
        }

        for (name, p) in params.into_iter() {
            if p.id == 0 {
                UserParam::add(user.id, name, &p.value).unwrap();
            } else {
                UserParam::update(p.id, name, &p.value).unwrap();
            }
        }
    } else {
        let id = user::add_user(user.domain_id, &user.user_id).unwrap();
        for (name, var) in vars.into_iter() {
            UserVariable::add(id, name, &var.value).unwrap();
        }
    }

    web::Json(Status {status: "Ok".to_string()})
}

async fn delete(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_,id) = path.into_inner();

    user::del_user(id).unwrap();

    web::Json(Status {status: "Ok".to_string()})
}
async fn get(path: web::Path<(i32, i32)>) -> impl Responder {
    let (domain_id,id) = path.into_inner();
    let user = user::get_user(ByField::Id(id)).unwrap_or(models::User {
        id: 0, domain_id, user_id: "".to_string()
    });

    let params = user::get_user_params(id).unwrap_or(vec![]);
    let vars = user::get_user_vars(id).unwrap_or(vec![]);

    let params = params.iter().map(|p| {
        (p.name.clone(), p.clone())
    }).collect::<HashMap<String, UserParam>>();

    let vars = vars.iter().map(|v| {
        (v.name.clone(), v.clone())
    }).collect::<HashMap<String, UserVariable>>();

    web::Json(User {
        id: user.id,
        domain_id: user.domain_id,
        user_id: user.user_id,
        params,
        vars
    })
}

async fn index(path: web::Path<i32>) -> impl Responder {
    let domain = path.into_inner();
    let users = user::users_within(domain).unwrap();

    web::Json(users.into_iter().map(|u|{
        User {
            id: u.id,
            user_id: u.user_id,
            domain_id: u.domain_id,
            params: HashMap::new(),
            vars: HashMap::new(),
        }
    }).collect::<Vec<User>>())
}
