use actix_web::{error::ErrorInternalServerError, web, Error};
use jlib::firewall;

pub fn firewall_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::get().to(index)));
}

async fn index() -> Result<web::Json<Vec<firewall::FirewallRule>>, Error> {
    firewall::list()
        .map(web::Json)
        .map_err(|err| ErrorInternalServerError(err.to_string()))
}
