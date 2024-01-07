use xml::writer::{EventWriter};
use std::io::Write;
use crate::fs::xml_utils::{start_element, end_element, attrs, param};

use fs_lib::queue;
use fs_lib::queue::agent;
use fs_lib::queue::tier;
use fs_lib::user;
use fs_lib::user::ByField;
use fs_lib::domain;

pub fn serve<W: Write>(w: &mut EventWriter<W>) {
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

fn agent_name(agent: &agent::Agent) -> String {
    let user = user::get_user(ByField::Id(agent.user_id)).unwrap();
    let domain = domain::get_domain(user.domain_id).unwrap();

    format!("{}@{}", user.user_id, domain.domain_name)
}

pub fn agent<W: Write>(w: &mut EventWriter<W>, agent: agent::Agent) {
    let params = agent::params(agent.id).unwrap();
    let name = agent_name(&agent);
    let contact = format!("[leg_timeout={}]user/{}", agent.leg_timeout, &name);

    let mut vec_params: Vec<(&str, &str)> = Vec::new();
    vec_params.push(("name", &name));
    vec_params.push(("contact", &contact));

    for p in &params {
        vec_params.push((&p.name, &p.value));
    }

    start_element(w, "agent", attrs(vec_params));
    end_element(w);
}

pub fn tiers <W: Write>(w: &mut EventWriter<W>) {
    start_element(w, "tiers", None);
    let tiers = tier::all().unwrap();
    for t in tiers {
        tier(w, t);
    }
    end_element(w);
}

pub fn tier<W: Write>(w: &mut EventWriter<W>, tier: tier::Tier) {
    let agent = agent::get(tier.agent_id).unwrap();
    let queue = queue::get(tier.queue_id).unwrap();

    let q_name = queue_name(&queue);
    let a_name = agent_name(&agent);
    start_element(w, "tier", attrs(vec![
        ("agent", &a_name),
        ("queue", &q_name),
        ("level", &tier.level.to_string()),
        ("position", &tier.position.to_string())
    ]));
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

fn queue_name(queue: &queue::Queue) -> String {
    let domain = domain::get_domain(queue.domain_id).unwrap();
    format!("{}@{}", queue.exten, domain.domain_name)
}

pub fn queue<W: Write>(w: &mut EventWriter<W>, queue: queue::Queue) {
    let name = queue_name(&queue);

    start_element(w, "queue", attrs(vec![("name", &name)]));
    let params = queue::params(queue.id).unwrap();
    for p in params {
        param(w, &p.name, &p.value);
    }
    end_element(w);
}
