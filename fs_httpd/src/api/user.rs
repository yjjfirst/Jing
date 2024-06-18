use std::collections::HashMap;
use actix_web::{web, Responder};
use fs_lib::{user};
use fs_lib::user::{ByField};
use fs_lib::user::models::User;
use fs_lib::user::user_param::UserParam;
use fs_lib::user::user_variable::UserVariable;
use serde::{Serialize, Deserialize};

use super::Status;
#[derive(Serialize, Deserialize)]
pub struct UserContainer {
    user: User,
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

async fn post(uc: web::Json<UserContainer>) -> impl Responder {
    let user = &uc.user;
    let vars = &uc.vars;
    let params = &uc.params;

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

    web::Json(Status {status: "Ok".to_string()})
}

async fn delete(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_,id) = path.into_inner();

    user::del_user(id).unwrap();

    web::Json(Status {status: "Ok".to_string()})
}
async fn get(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_,id) = path.into_inner();
    let user = user::get_user(ByField::Id(id)).unwrap();
    let params = user::get_user_params(id).unwrap();
    let vars = user::get_user_vars(id).unwrap();

    let params = params.iter().map(|p| {
        (p.name.clone(), p.clone())
    }).collect::<HashMap<String, UserParam>>();

    let vars = vars.iter().map(|v| {
        (v.name.clone(), v.clone())
    }).collect::<HashMap<String, UserVariable>>();

    web::Json(UserContainer {
        user, params, vars
    })
}

async fn index(path: web::Path<i32>) -> impl Responder {
    let domain = path.into_inner();
    let users = user::users_within(domain).unwrap();
    web::Json(users)
}
