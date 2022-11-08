use xml::writer::{EventWriter};
use std::io::Write;
use crate::xml_utils::{start_element, end_element, attrs, param};

use fslib::queue;
use fslib::queue::agent;
use fslib::user;

pub fn serve<W: Write>(w: &mut EventWriter<W>) {
    start_element(w, "document", attrs(vec![("type", "freeswitch/xml")]));
    start_element(w, "section", attrs(vec![("name", "configuration")]));

    start_element(w,
                  "configuration",
                  attrs(vec![
                      ("name","callcenter.conf"),
                      ("description", "CallCenter")
                  ]));
    settings(w);
    queues(w);
    agents(w);
    tiers(w);

    end_element(w);
    end_element(w);
    end_element(w);
}

pub fn settings <W: Write>(w: &mut EventWriter<W>) {
    start_element(w, "settings", None);
    end_element(w);
}

pub fn agents <W: Write>(w: &mut EventWriter<W>) {
    start_element(w, "agents", None);
    let agents = agent::all().unwrap();

    for a in agents {
        agent(w, a);
    }

    end_element(w);
}

pub fn agent<W: Write>(w: &mut EventWriter<W>, agent: agent::Agent) {
    let params = agent::params(agent.id).unwrap();
    let user = user::get_user(agent.user_id).unwrap();
    let contact = format!("[leg_timeout={}]user/{}", agent.leg_timeout, user.user_id);

    let mut vec_params: Vec<(&str, &str)> = Vec::new();
    vec_params.push(("name", &agent.name));
    vec_params.push(("contact", &contact));

    for p in &params {
        vec_params.push((&p.name, &p.value));
    }

    start_element(w, "agent", attrs(vec_params));
    end_element(w);
}

pub fn tiers <W: Write>(w: &mut EventWriter<W>) {
    start_element(w, "tiers", None);
    end_element(w);
}

pub fn queues<W: Write>(w: &mut EventWriter<W>) {
    let queues = queue::all().unwrap();
    start_element(w, "queues", None);
    for q in queues {
        queue(w, q);
    }
    end_element(w);
}

pub fn queue<W: Write>(w: &mut EventWriter<W>, queue: queue::Queue) {
    start_element(w, "queue", attrs(vec![("name", &queue.name)]));
    let params = queue::params(queue.id).unwrap();
    for p in params {
        param(w, &p.name, &p.value);
    }
    end_element(w);
}
