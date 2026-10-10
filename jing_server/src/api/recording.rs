use std::ops::Deref;
use actix_web::{web, Responder};
use actix_multipart::form::{text::Text, tempfile::TempFile, MultipartForm};

use jing_lib::recording;
use jing_lib::recording::models::{Recording};
use super::Status;

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "20MB")]
    file_name: TempFile,
    description: Text<String>,
    domain_id: Text<i32>
}

pub fn recording_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("")
                .route(web::get().to(index)))
        .service(
            web::resource("/{id}")
                .route(web::get().to(get))
                .route(web::post().to(post))
                .route(web::patch().to(patch))
                .route(web::delete().to(delete))
        );
}

async fn index() -> impl Responder {
    let files = recording::list().unwrap();
    web::Json(files)
}

async fn get(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_,id) =path.into_inner();
    let s = recording::get(id).unwrap_or(Recording {
        id: 0,
        domain_id: 0,
        name:"".to_string(),
        description: Some("".to_string())
    });

    web::Json(s)
}

async fn delete(path: web::Path<(i32, i32)>) -> impl Responder {
    let (_,id) =path.into_inner();
    recording::del(id).unwrap();

    web::Json(Status{status: "Ok".to_string()})
}

async fn post(MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
    let f = form.file_name;
    let path = format!("/tmp/{}", f.file_name.clone().unwrap());

    f.file.persist(path.clone()).unwrap();
    recording::add(
        form.domain_id.into_inner(),
        f.file_name.unwrap(),
        path,
        form.description.into_inner()).unwrap();

    web::Json(Status{status: "Ok".to_string()})
}


async fn patch(s: web::Json<Recording>) -> impl Responder {
    let r = s.deref();
    recording::update(r.clone()).unwrap();

    web::Json(Status {status: "Ok".to_string()})
}
