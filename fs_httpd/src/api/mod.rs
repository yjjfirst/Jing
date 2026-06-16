pub mod ring_group;
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
pub mod ivr;
pub mod callcenter;
pub mod profile;
pub mod login;
pub mod logout;

use actix_web::{web};
use ring_group::ring_group_config;
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
use ivr::ivr_config;
use callcenter::cc_config;
use profile::profile_config;
use login::login_config;
use logout::logout_config;
use serde::Serialize;

#[derive(Serialize)]
struct Status {
    status: String,
}
pub fn api_config(cfg: &mut web::ServiceConfig) {

    cfg
        .service(web::scope("/{domain}/gateway").configure(gateway_config))
        .service(web::scope("/{domain}/user").configure(user_config))
        .service(web::scope("/{domain}/ring-group").configure(ring_group_config))
        .service(web::scope("/{domain}/outbound").configure(outbound_config))
        .service(web::scope("/{domain}/inbound").configure(inbound_config))
        .service(web::scope("/{domain}/cdr").configure(cdr_config))
        .service(web::scope("/{domain}/sound-file").configure(sound_file_config))
        .service(web::scope("/{domain}/extension").configure(extension_config))
        .service(web::scope("/{domain}/sound").configure(sound_config))
        .service(web::scope("/{domain}/conference").configure(conf_config))
        .service(web::scope("/{domain}/ivr").configure(ivr_config))
        .service(web::scope("/{domain}/callcenter").configure(cc_config))
        .service(web::scope("/{domain}/profile").configure(profile_config))
        .service(web::scope("/domain").configure(domain_config))
        .service(web::scope("/login").configure(login_config))
        .service(web::scope("/logout").configure(logout_config));
}
