use actix_web::{post, web, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};

#[derive(Debug, Deserialize)]
pub struct CdrXml {
    cdr: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Variables {
    direction: String,
    sip_from_user: String,
    sip_to_user: String,
    duration: i32,
    billsec: i32,
    start_stamp: String
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Cdr {
    variables: Variables,
}

#[post("/cdr")]
pub async fn cdr_post(req: web::Form<CdrXml>) -> impl Responder {
    println!("{}", req.cdr);
    let cdr: Cdr = from_str(&req.cdr).unwrap();
    println!("{:?}", cdr);
    HttpResponse::Ok().body("ok")
}
