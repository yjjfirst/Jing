extern crate fslib;

use std::io::Write;
use std::io::BufWriter;
use xml::writer::{EmitterConfig, EventWriter};
use super::xml_utils::{start_element, end_element, param, variable, Attr};

use fslib::domain;
use fslib::domain::models::Domain;
use fslib::user::{all_users};
use fslib::user::models::{User};
use fslib::voicemail::{get_voicemail};

pub fn serve () -> tide::Result {
    let mut buf = BufWriter::new(Vec::new());
    let mut w = EmitterConfig::new().perform_indent(true).create_writer(&mut buf);

    start_element(&mut w, "document", Some(vec![Attr {name: "type", value: "freeswitch/xml"}]));
    start_element(&mut w, "section", Some(vec![Attr{name: "name", value: "directory"}]));

    domains(&mut w);
    end_element(&mut w);
    end_element(&mut w);

    let response = buf.into_inner().unwrap();
    let response: String = String::from_utf8(response).unwrap();

    Ok(response.into())

}

fn domains<W: Write>(w: &mut EventWriter<W>) {
    let domains = domain::list_domains().unwrap();
    for d in domains {
        domain(w, d);
    }
}

fn domain<W: Write>(w: &mut EventWriter<W>, domain: Domain) {
    start_element(w, "domain", Some(vec![Attr::new("name",  domain.domain_name.as_str())]));

    start_element(w, "params", None);
    param(w, "dial-string", "${sofia_contact(${dialed_user}@${dialed_domain})}");
    end_element(w);

    users(w, domain);

    end_element(w);
}

fn users<W: Write>(w: &mut EventWriter<W>, domain: Domain) {
    let users = all_users().unwrap();
    for u in users {
        if u.domain_id == domain.id {
            user(w, u);
        }
    }
}

fn user<W: Write>(w: &mut EventWriter<W>, u: User) {
    start_element(w, "user", Some(vec![Attr::new("id", u.user_id.as_str())]));
    user_params(w, &u);
    user_variables(w, &u);
    end_element(w);
}

fn user_params<W: Write>(w: &mut EventWriter<W>, u: &User) {
    start_element(w, "params", None);

    param(w, "password", u.password.as_str());
    let vm = get_voicemail(u.id);
    if let Ok(v) = vm {
        param(w, "vm-password", &v.password);
    }

    end_element(w);
}

fn user_variables<W: Write>(w: &mut EventWriter<W>, u: &User) {
    start_element(w, "variables", None);
    variable(w, "user_context", "internal");
    variable(w, "effective_caller_id_name", &u.effective_caller_id_name.as_ref().unwrap_or(&u.user_id));
    variable(w, "effective_caller_id_number", &u.effective_caller_id_number.as_ref().unwrap_or(&u.user_id));
    variable(w, "outbound_caller_id_name", &u.outbound_caller_id_name.as_ref().unwrap_or(&u.user_id));
    variable(w, "outbound_caller_id_number", &u.outbound_caller_id_number.as_ref().unwrap_or(&u.user_id));

    end_element(w);
}
