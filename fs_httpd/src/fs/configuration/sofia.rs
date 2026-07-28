use xml::writer::{EventWriter};
use std::io::Write;
use fs_lib::gateway::models::{Gateway};
use fs_lib::gateway::{get_params};
use fs_lib::profile::models::{Profile};
use fs_lib::profile;

use crate::fs::xml_utils::{start_element, end_element, param, Attr};

pub fn serve<W: Write>(w: &mut EventWriter<W>) {
    start_element(w, "configuration", Some(
        vec![Attr {name: "name", value: "sofia.conf"},
             Attr {name: "description",value: "sofia Endpoint"}]));

    profiles(w);

    end_element(w);

}


fn profiles<W: Write>(w: &mut EventWriter<W>) {
    start_element(w,"profiles", None);

    let profiles = profile::list().unwrap();
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
    start_element(w,
                  "gateway",
                  Some(vec![
                      Attr {
                          name: "name",
                          value: g.gateway_name.as_str()
                      }]));

    let params = get_params(g.id).unwrap();

    for p in &params {
        param(w, &p.name, &p.value);
    }

    end_element(w)
}

fn settings<W: Write>(w: &mut EventWriter<W>, profile_name: &str) {
    start_element(w, "settings", None);
    let profile = profile::get_profile_by_name(profile_name).unwrap();
    let params = profile::get_profile_params(profile.id).unwrap();
    for p in params {
        param(w, p.name.as_str(), p.value.as_str());
    }
    end_element(w);
}
