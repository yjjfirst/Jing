pub mod ringing_group;
pub mod domain;
pub mod user;
pub mod gateway;
pub mod outbound;
pub mod inbound;
pub mod cdr;
pub mod sound_file;
pub mod extension;
pub mod sound;
pub mod conference;

use actix_web::{web};
use ringing_group::ringing_group_config;
use domain::domain_config;
use gateway::gateway_config;
use user::user_config;
use outbound::outbound_config;
use inbound::inbound_config;
use sound_file::sound_file_config;
use cdr::cdr_config;
use extension::extension_config;
use sound::sound_config;
use conference::conf_config;
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
        .service(web::scope("/{domain}/cdr").configure(cdr_config))
        .service(web::scope("/{domain}/sound-file").configure(sound_file_config))
        .service(web::scope("/{domain}/extension").configure(extension_config))
        .service(web::scope("/{domain}/sound").configure(sound_config))
        .service(web::scope("/{domain}/conference").configure(conf_config))
        .service(web::scope("/domain").configure(domain_config));
}
