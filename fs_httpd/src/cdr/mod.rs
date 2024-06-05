use actix_web::{post, web, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

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

    let subject = format!("Call from {} to {}.",
                          cdr.variables.sip_from_user,
                          cdr.variables.sip_to_user);

    let body = format!("Hi Martin, \n\nYou have call from {} to {}. \nPlease take action if you didn't make the calls.\n\nBR,\nFSRust",
                       cdr.variables.sip_from_user,
                       cdr.variables.sip_to_user);

    let email = Message::builder()
        .from("NoReply <yjjfirst@gmail.com>".parse().unwrap())
        .to("Martin Yang <yjjfirst@hotmail.com>".parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(body))
        .unwrap();

    let creds = Credentials::new("yjjfirst@gmail.com".to_owned(), "quissatpxjnlnlak ".to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => println!("Could not send email: {e:?}"),
    }

    HttpResponse::Ok().body("ok")
}
