extern crate fslib;

use std::io::Write;
use std::io::BufWriter;
use xml::writer::{EmitterConfig, EventWriter};
use super::xml_utils::{start_element, end_element, param, Attr};
use fslib::gateway::models::{Gateway};
use fslib::profile::models::{Profile};

use fslib::profile;

pub fn serve () -> tide::Result {
    let mut buf = BufWriter::new(Vec::new());
    let mut w = EmitterConfig::new().perform_indent(true).create_writer(&mut buf);

    start_element(&mut w, "document", Some(vec![Attr {name: "type", value: "freeswitch/xml"}]));
    start_element(&mut w, "section", Some(vec![Attr{name: "name", value: "configuration"}]));
    start_element(&mut w, "configuration", Some(
        vec![Attr {name: "name", value: "sofia.conf"},
             Attr {name: "description",value: "sofia Endpoint"}]));

    profiles(&mut w);

    end_element(&mut w);
    end_element(&mut w);
    end_element(&mut w);

    let response = buf.into_inner().unwrap();
    let response: String = String::from_utf8(response).unwrap();

    Ok(response.into())
}

fn profiles<W: Write>(w: &mut EventWriter<W>) {
    start_element(w,"profiles", None);

    let profiles = profile::all_profiles().unwrap();
    for p in profiles {
        profile(w, p);
    }

    end_element(w);
}

fn profile<W: Write>(w: &mut EventWriter<W>, profile: Profile) {
    start_element(w, "profile", Some(vec![Attr{name: "name", value: profile.name.as_str()}]));
    gateways(w, profile.id);
    settings(w, profile.name.as_str());
    end_element(w);
}

fn gateways<W: Write>(w: &mut EventWriter<W>, profile_id: i32) {
    let gateways = profile::gateways(profile_id).unwrap();

    start_element(w, "gateways", None);
    for g in &gateways {
        gateway(w, g);
    }
    end_element(w);
}

fn gateway<W: Write>(w: &mut EventWriter<W>, g: &Gateway) {
    start_element(w, "gateway", Some(vec![Attr {name: "name", value: g.gateway_name.as_str()}]));

    param(w, "proxy", g.proxy.as_str());
    param(w, "username", g.username.as_ref().unwrap());
    param(w, "password", g.password.as_ref().unwrap());
    param(w, "register", g.register.as_str());

    end_element(w)
}

fn settings<W: Write>(w: &mut EventWriter<W>, profile_name: &str) {
    start_element(w, "settings", None);
    let params = profile::profile_params(profile_name.to_string()).unwrap();
    for p in params {
        param(w, p.name.as_str(), p.value.as_str());
    }
    end_element(w);
}
