pub mod ringing_group;
pub mod domain;

use actix_web::{web, Responder};
use ringing_group::ringing_group_config;
use domain::domain_config;

pub fn api_config(cfg: &mut web::ServiceConfig) {

    cfg
        .service(web::scope("/ringing-groups").configure(ringing_group_config))
        .service(web::scope("/domain").configure(domain_config));
}
