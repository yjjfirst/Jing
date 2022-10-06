extern crate fslib;

use std::io::Write;
use std::io::BufWriter;
use xml::writer::{EmitterConfig, EventWriter};
use super::xml_utils::{start_element, end_element, action, Attr};
use fslib::extension::{get_extension};
use fslib::route::{all_outbounds, all_inbound};
use fslib::route::outbound_models::{OutboundRoute};
use fslib::route::inbound_models::{InboundRoute};
use fslib::gateway::{get_gateway};
use fslib::ringgroup::{all_ringgroup, all_ringgroup_member};
use fslib::ringgroup::models::{Ringgroup};
use fslib::domain::{get_domain_by_name};
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

    if context == "internal" {
        let dest_number = fs_req.dest_number.unwrap();
        let domain_name = fs_req.dest_domain.unwrap();
        let domain = get_domain_by_name(domain_name).unwrap();

        if let Ok(e) = get_extension(dest_number.as_str(), domain.id) {
            if e.exten_type == "user" {
                user(w);
            } else if e.exten_type == "ringgroup" {
                ringgroups(w);
            } else if e.exten_type == "sound" {
            }
        } else {
            outbounds(w);
        }
    } else if context == "public" {
        inbounds(w);
    }
}

fn user<W: Write>(w: &mut EventWriter<W>)  {
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

fn outbounds<W: Write>(w: &mut EventWriter<W>) {
    start_element(w, "context", Some(vec![Attr::new("name", "internal")]));
    for route in all_outbounds().unwrap() {
        outbound(w, route);
    }
    end_element(w);
}

fn outbound<W: Write>(w: &mut EventWriter<W>, route: OutboundRoute) {
    start_element(w, "extension", Some(vec![Attr::new("name", format!("outbound_route_{}", route.id).as_str())]));
    start_element(w, "condition", Some(vec![Attr::new("field", "destination_number"),
                                            Attr::new("expression", route.condition.as_str())
    ]));

    if let Ok(g) = get_gateway(route.gateway_id) {
        action(w,"bridge", format!("sofia/gateway/{}/$1",g.gateway_name).as_str());
    }
    end_element(w);
    end_element(w);
}

fn inbounds<W: Write>(w: &mut EventWriter<W>) {
    start_element(w, "context", Some(vec![Attr::new("name", "public")]));
    for r in all_inbound().unwrap() {
        inbound(w, r);
    }
    end_element(w);
}

fn inbound<W: Write>(w: &mut EventWriter<W>, route: InboundRoute) {
    start_element(w, "extension", Some(vec![Attr::new("name", format!("inbound_route_{}", route.id).as_str())]));
    start_element(w, "condition", Some(vec![Attr::new("field", "destination_number"),
                                            Attr::new("expression", route.condition.as_str())
    ]));

    action(w, "transfer", format!("{} XML internal", route.dest_extension).as_str());

    end_element(w);
    end_element(w);

}

fn ringgroups<W: Write>(w: &mut EventWriter<W>) {
    start_element(w, "context", Some(vec![Attr::new("name", "internal")]));
    for rg in all_ringgroup().unwrap() {
        ringgroup(w, rg);
    }
    end_element(w);
}

fn ringgroup<W: Write>(w: &mut EventWriter<W>, rg: Ringgroup) {
    start_element(w, "extension", Some(vec![Attr::new("name", format!("ringgroup_{}", rg.id).as_str())]));
    start_element(w, "condition", Some(vec![Attr::new("field", "destination_number"),
                                            Attr::new("expression", rg.group_id.as_str())
    ]));

    let members = all_ringgroup_member(rg.id).unwrap();
    let members: Vec<String> = members.iter().map(|m| format!("user/{}@${{domain_name}}",m.1)).collect();
    let members = members.join(",");

    action(w, "set", "call_timeout=30");
    action(w, "bridge", format!("{{ignore_early_media=true}}{}", members).as_str());

    end_element(w);
    end_element(w);
}
