mod sofia;
mod conference;
mod callcenter;
mod ivr;

extern crate fs_lib;

use std::io::BufWriter;
use xml::writer::{EmitterConfig};
use super::xml_utils::*;
use super::FsRequest;
use actix_web::Result;

pub fn serve (fs_req: FsRequest) -> Result<String> {
    let mut buf = BufWriter::new(Vec::new());
    let mut w = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(&mut buf);
    start_element(&mut w, "document", Some(vec![Attr {name: "type", value: "freeswitch/xml"}]));
    start_element(&mut w, "section", Some(vec![Attr{name: "name", value: "configuration"}]));

    if fs_req.key_value == "sofia.conf" {
        sofia::serve(&mut w);
    } else if fs_req.key_value == "conference.conf" {
        conference::serve(&mut w);
    } else if fs_req.key_value == "callcenter.conf" {
        callcenter::serve(&mut w);
    } else if fs_req.key_value == "ivr.conf" {
        ivr::serve(&mut w);
    }

    end_element(&mut w);
    end_element(&mut w);

    let response = buf.into_inner().unwrap();
    let response: String = String::from_utf8(response).unwrap();

    Ok(response.into())
}
