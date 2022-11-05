use xml::writer::{EventWriter};
use std::io::Write;
use crate::xml_utils::{start_element, end_element, attrs, param};

use fslib::queue;

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
