mod sofia;
mod conference;
mod callcenter;

extern crate fslib;

use std::io::BufWriter;
use xml::writer::{EmitterConfig};

use super::FsRequest;

pub fn serve (fs_req: FsRequest) -> tide::Result {
    let mut buf = BufWriter::new(Vec::new());
    let mut w = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(&mut buf);
    if fs_req.key_value == "sofia.conf" {
        sofia::serve(&mut w);
    } else if fs_req.key_value == "conference.conf" {
        conference::serve(&mut w);
    } else if fs_req.key_value == "callcenter.conf" {
        callcenter::serve(&mut w);
    }

    let response = buf.into_inner().unwrap();
    let response: String = String::from_utf8(response).unwrap();

    Ok(response.into())
}
