use actix_web::{web, Responder};
use fs_lib::ringgroup;

pub fn ringing_group_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/ringing-groups")
            .route(web::get().to(ringing_groups))
    );
}

async fn ringing_groups() ->  impl Responder {
    let groups = ringgroup::all_ringgroup().unwrap();
    web::Json(groups)
}
