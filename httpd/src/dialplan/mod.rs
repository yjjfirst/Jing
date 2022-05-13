extern crate fslib;

use std::io::Write;
use std::io::BufWriter;
use xml::writer::{EmitterConfig, EventWriter};
use super::xml_utils::{start_element, end_element, action, Attr};
use fslib::extension::{get_extension};

use super::FsRequest;

pub fn serve (fs_req: FsRequest) -> tide::Result {
    let mut buf = BufWriter::new(Vec::new());
    let mut w = EmitterConfig::new().perform_indent(true).create_writer(&mut buf);

    start_element(&mut w, "document", Some(vec![Attr {name: "type", value: "freeswitch/xml"}]));
    start_element(&mut w, "section", Some(vec![Attr::new("name", "dialplan"),
                                               Attr::new("description", "Dialplan for Freeswitch")]));
    dialplan(&mut w, fs_req);

    end_element(&mut w);
    end_element(&mut w);

    let response = buf.into_inner().unwrap();
    let response: String = String::from_utf8(response).unwrap();

    Ok(response.into())
}

fn dialplan<W: Write>(w: &mut EventWriter<W>, fs_req: FsRequest) {
    let context = fs_req.context.unwrap();
    let dest_number = fs_req.dest_number.unwrap();
    let domain = fs_req.dest_domain.unwrap();

    if context == "internal" {
        if let Ok(e) = get_extension(dest_number.as_str()) {
            if e.exten_type == "user" {
                user(w, e.exten.as_str(), domain.as_str());
            }
        }
    }
}

fn user<W: Write>(w: &mut EventWriter<W>, _user: &str, _domain: &str)  {
    start_element(w, "context", Some(vec![Attr::new("name", "internal")]));
    start_element(w, "extension", Some(vec![Attr::new("name", "local_user")]));
    start_element(w, "condition", Some(vec![Attr::new("field", "destination_number"),
                                            Attr::new("expression", "^(.*)$"),
    ]));

    action(w, "export","dialed_extension=$1");
    action(w, "set","call_timeout=30");
    action(w, "set","hangup_after_bridge=true");
    action(w, "set","continue_on_fail=true");
    action(w, "bridge", "user/${dialed_extension}@${domain_name}");

    end_element(w);
    end_element(w);
    end_element(w);
}
