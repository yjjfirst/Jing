use actix_web::{web, Responder};
use fs_lib::{user};
use fs_lib::user::{ByField};
use fs_lib::user::models::User;
use fs_lib::user::user_param::UserParam;
use fs_lib::user::user_variable::UserVariable;
use serde::Serialize;

#[derive(Serialize)]
pub struct WebUser {
    user: User,
    vars: Vec<UserVariable>,
    params: Vec<UserParam>,
}

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("")
                .route(web::get().to(index)))
        .service(
            web::resource("/{id}")
                .route(web::get().to(get)));

}

async fn get(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_,id) = path.into_inner();
    let user = user::get_user(ByField::Id(id)).unwrap();
    let params = user::get_user_params(id).unwrap();
    let vars = user::get_user_vars(id).unwrap();

    web::Json(WebUser {
        user, params, vars
    })
}

async fn index(path: web::Path<i32>) -> impl Responder {
    let domain = path.into_inner();
    let users = user::users_within(domain).unwrap();
    web::Json(users)
}
