use chrono::{TimeZone,Local};
use actix_web::{post, web, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str};

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use jlib::cdr;
use jlib::system_setting;

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
    uuid: String,
    start_epoch: String,
    answer_epoch: String,
    end_epoch: String,
    hangup_cause: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CallerProfile {
    caller_id_name: String,
    caller_id_number: String,
    destination_number: String
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Callflow {
    caller_profile: CallerProfile
}
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Cdr {
    variables: Variables,
    callflow_first: Callflow,
}

#[post("/cdr")]
pub async fn cdr_post(req: web::Form<CdrXml>) -> impl Responder {
    let cdr_string: String = req.cdr.replacen("callflow", "callflow_first", 2);
    let cdr: Cdr = from_str(&cdr_string).unwrap();

    send_email(
        &cdr.callflow_first.caller_profile.caller_id_number,
        &cdr.callflow_first.caller_profile.destination_number,
    );


    cdr::add_cdr (
        cdr.callflow_first.caller_profile.caller_id_number,
        cdr.callflow_first.caller_profile.caller_id_name,
        cdr.callflow_first.caller_profile.destination_number,
        Local.timestamp_opt(cdr.variables.start_epoch.parse::<i64>().unwrap(), 0).unwrap(),
        if cdr.variables.answer_epoch.parse::<i64>().unwrap() == 0 {
            None
        } else {
            Some(Local.timestamp_opt(cdr.variables.answer_epoch.parse::<i64>().unwrap(), 0).unwrap())
        },
        Local.timestamp_opt(cdr.variables.end_epoch.parse::<i64>().unwrap(), 0).unwrap(),
        cdr.variables.duration,
        cdr.variables.billsec,
        cdr.variables.hangup_cause
    ).unwrap();

    HttpResponse::Ok().body("ok")
}

pub fn send_email(from: &str, to: &str) {
    let smtp_username = system_setting::get("smtp", "smtp_username").unwrap();
    let smtp_password = system_setting::get("smtp", "smtp_password").unwrap();
    let smtp_host = system_setting::get("smtp", "smtp_host").unwrap();
    let admin_email = system_setting::get("admin", "admin_email").unwrap();

    let subject = format!("Call from {} to {}.",
                          from,
                          to);

    let body = format!("Hi Martin, \n\nYou have call from {} to {}. \nPlease take action if you didn't make the calls.\n\nBR,\nFSRust",
                       from,
                       to);

    let email = Message::builder()
        .from(format!("NoReply <{}>", smtp_username).parse().unwrap())
        .to(format!("Martin Yang <{}>", admin_email).parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(body))
        .unwrap();

    let creds = Credentials::new(smtp_username, smtp_password);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&smtp_host)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => println!("Could not send email: {e:?}"),
    }

}
