mod xml_utils;
mod configuration;
mod directory;
mod dialplan;

use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FsRequest {
    hostname: String,
    section: String,
    tag_name: String,
    key_name: String,
    key_value: String,
    #[serde(rename = "Caller-Destination-Number")]
    dest_number: Option<String>,
    #[serde(rename = "Caller-Context")]
    context: Option<String>,
    #[serde(rename = "variable_sip_to_host")]
    dest_domain: Option<String>,
    #[serde(rename = "Caller-Caller-ID-Number")]
    caller_id: Option<String>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new().service(fs_post)
    })
        .bind("127.0.0.1:9090")?
        .bind("45.76.77.24:9090")?
        .run()
        .await
}

#[post("/fsapi")]
async fn fs_post(req: web::Form<FsRequest>) -> impl Responder {
    println!("{:?}", req);
    println!("{}", req.hostname);

    if req.section == "configuration" {
        HttpResponse::Ok().body(configuration::serve(req.0).unwrap())
    } else if req.section == "directory" {
        HttpResponse::Ok().body(directory::serve().unwrap())
    } else if req.section == "dialplan" {
        HttpResponse::Ok().body(dialplan::serve(req.0).unwrap())
    } else {
        HttpResponse::Ok().body("invalid url")
    }
}
