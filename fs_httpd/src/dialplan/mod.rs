mod feature_code;
extern crate fs_lib;

use std::io::Write;
use std::io::BufWriter;
use xml::writer::{EmitterConfig, EventWriter};
use super::xml_utils::{start_element, end_element, action, Attr};
use fs_lib::extension::{get_extension, Extension};
use fs_lib::route::{all_outbounds, all_inbound};
use fs_lib::route::outbound_models::{OutboundRoute};
use fs_lib::route::inbound_models::{InboundRoute};
use fs_lib::gateway;
use fs_lib::ringgroup;
use fs_lib::domain;
use fs_lib::sound;
use fs_lib::sound_file;
use fs_lib::conference;
use fs_lib::queue;
use fs_lib::ivr;

use super::FsRequest;

pub fn serve (fs_req: FsRequest) -> tide::Result {
    let mut buf = BufWriter::new(Vec::new());
    let mut w = EmitterConfig::new().perform_indent(true).create_writer(&mut buf);

    start_element(&mut w, "document",
                  Some(vec![Attr {name: "type", value: "freeswitch/xml"}]));
    start_element(&mut w, "section",
                  Some(vec![Attr::new("name", "dialplan"),
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
        let dest_exten = fs_req.dest_number.unwrap();
        let domain_name = fs_req.dest_domain.unwrap();
        let domain = domain::get_domain_by_name(domain_name).unwrap();

        if let Ok(e) = get_extension(dest_exten.as_str(), domain.id) {
            start_internal_xml(w, &e);

            if e.exten_type == "user" {
                user(w);
            } else if e.exten_type == "ringgroup" {
                ringgroup(w, domain.id, dest_exten);
            } else if e.exten_type == "sound" {
                sound(w, domain.id, dest_exten);
            } else if e.exten_type == "conference" {
                conference(w, domain.id, dest_exten);
            } else if e.exten_type == "queue" {
                queue(w, domain.id, dest_exten);
            } else if e.exten_type == "ivr" {
                ivr(w, domain.id, dest_exten);
            } else if e.exten_type == "feature_code" {
                feature_code::serve(w, dest_exten, fs_req.caller_id);
            }

            end_internal_xml(w);
        } else {
            outbounds(w);
        }
    } else if context == "public" {
        inbounds(w);
    }
}

fn start_internal_xml<W: Write>(w: &mut EventWriter<W>, exten: &Extension) {
    let name = format!("{}_{}",exten.exten_type,exten.exten);
    start_element(w, "context", Some(vec![Attr::new("name", "internal")]));
    start_element(w, "extension", Some(vec![Attr::new("name", &name)]));
    start_element(w, "condition", Some(vec![Attr::new("field", "destination_number"),
                                            Attr::new("expression", "^(.*)$"),
    ]));
}

fn end_internal_xml<W: Write>(w: &mut EventWriter<W>) {
    end_element(w);
    end_element(w);
    end_element(w);
}

fn user<W: Write>(w: &mut EventWriter<W>)  {
    action(w, "export","dialed_extension=$1");
    action(w, "set","call_timeout=30");
    action(w, "set","hangup_after_bridge=true");
    action(w, "set","continue_on_fail=true");
    action(w, "bridge", "user/${dialed_extension}@${domain_name}");
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

    if let Ok(g) = gateway::get_gateway(route.gateway_id) {
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

fn ringgroup<W: Write>(w: &mut EventWriter<W>, domain_id: i32, exten: String) {
    let rg = ringgroup::get_by(domain_id, &exten).unwrap();
    let members = ringgroup::all_ringgroup_member(rg.id).unwrap();
    let members: Vec<String> = members.iter().map(|m| format!("user/{}@${{domain_name}}",m.1)).collect();
    let members = members.join(",");

    action(w, "set", "call_timeout=30");
    action(w, "bridge", format!("{{ignore_early_media=true}}{}", members).as_str());
}


fn sound<W: Write>(w: &mut EventWriter<W>, domain_id: i32, exten: String) {
    let sound = sound::get_by(domain_id, &exten);
    if let Ok(s) = sound {
        let file = sound_file::get(s.sound_file_id);
        if let Ok(f) = file {
            action(w, "answer","");
            action(w, "sleep", "1000");
            action(w, "playback", &f.name);
        }
    };
}


fn conference<W: Write>(w: &mut EventWriter<W>, domain_id: i32, exten: String) {
    let conference = conference::get_by(domain_id, &exten);

    if let Ok(c) = conference {
        let profile = conference::conference_profile::get(c.conference_profile_id).unwrap();
        action(w, "Answer", "");
        let data = format!("{}-$${{domain_name}}@{}",&c.exten, &profile.name);
        action(w, "conference",  &data);
    }

}

fn queue<W: Write>(w: &mut EventWriter<W>, domain_id: i32, exten: String) {
    let queue = queue::get_by(domain_id, &exten);

    if let Ok(_q) = queue {
        let data = format!("{}@{}", exten, "$${domain}");
        action(w, "callcenter",  &data);
    }

}

fn ivr<W: Write>(w: &mut EventWriter<W>, domain_id: i32, exten: String) {
    let ivr = ivr::get_by(domain_id, &exten).unwrap();
    action(w, "ivr",  &ivr.name);
}
