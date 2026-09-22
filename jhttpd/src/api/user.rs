use std::collections::HashMap;
use actix_web::{web, Responder};
use jlib::{user};
use jlib::user::{ByField};
use jlib::user::models;
use jlib::user::user_param::UserParam;
use jlib::user::user_variable::UserVariable;
use jlib::domain::{get_domain};
use jlib::fs::sofia::{reg};
use serde::{Serialize, Deserialize};


use super::Status;
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub domain_id: i32,
    pub user_id: String,
    pub status: HashMap<String, String>,
    pub vars: HashMap<String, UserVariable>,
    pub params: HashMap<String, UserParam>,
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
        status: HashMap::new(),
        params,
        vars
    })
}

async fn index(path: web::Path<i32>) -> impl Responder {
    let domain_id = path.into_inner();
    let users = user::users_within(domain_id).unwrap();
    let domain  = get_domain(domain_id).unwrap();
    let users_status = reg(domain.domain_name);

    web::Json(users.into_iter().map(|u|{
        let user_status = users_status.get(&u.user_id);
        let status_map: HashMap<String, String > = match user_status {
            Some(s) => {
                HashMap::from([
                    ("ip_addr".to_string(), s.ip_addr.clone()),
                    ("agent".to_string(), s.agent.clone()),
                    ("ping".to_string(), s.ping.clone())
                    ])
            },
            None => {
                HashMap::new()
            }

        };

        User {
            id: u.id,
            user_id: u.user_id,
            domain_id: u.domain_id,
            status: status_map,
            params: HashMap::new(),
            vars: HashMap::new(),
        }
    }).collect::<Vec<User>>())
}
