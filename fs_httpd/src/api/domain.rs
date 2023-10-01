use actix_web::{web,Responder};
use fs_lib::domain;
use fs_lib::domain::models::Domain;
use fs_lib::rt::{eval, is_var};

pub fn domain_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("")
                .route(web::get().to(index)));
}

async fn index() -> impl Responder {
    let domains = domain::list_domains().unwrap();
    let domains: Vec<Domain> = domains.into_iter().map(|d| {
        let name = if is_var(&d.domain_name) {
            eval(&d.domain_name)
        } else {
            d.domain_name
        };

        Domain {
            domain_name: name,
            id: d.id
        }

    }).collect();

    web::Json(domains)
}
