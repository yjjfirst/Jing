pub mod ringing_group;
pub mod domain;
pub mod user;
pub mod gateway;
pub mod outbound;
pub mod inbound;

use actix_web::{web};
use ringing_group::ringing_group_config;
use domain::domain_config;
use gateway::gateway_config;
use user::user_config;
use outbound::outbound_config;
use inbound::inbound_config;
use serde::Serialize;

#[derive(Serialize)]
struct Status {
    status: String,
}
pub fn api_config(cfg: &mut web::ServiceConfig) {

    cfg
        .service(web::scope("/{domain}/gateway").configure(gateway_config))
        .service(web::scope("/{domain}/user").configure(user_config))
        .service(web::scope("/{domain}/ringing-group").configure(ringing_group_config))
        .service(web::scope("/{domain}/outbound").configure(outbound_config))
        .service(web::scope("/{domain}/inbound").configure(inbound_config))
        .service(web::scope("/domain").configure(domain_config));
}
