pub mod ringing_group;
pub mod domain;
pub mod user;

use actix_web::{web};
use ringing_group::ringing_group_config;
use domain::domain_config;
use user::user_config;
use serde::Serialize;

#[derive(Serialize)]
struct Status {
    status: String,
}
pub fn api_config(cfg: &mut web::ServiceConfig) {

    cfg
        .service(web::scope("/{domain}/user").configure(user_config))
        .service(web::scope("/{domain}/ringing-group").configure(ringing_group_config))
        .service(web::scope("/domain").configure(domain_config));

}
