extern crate fslib;

use xml::writer::{EventWriter};
use std::io::Write;
use crate::xml_utils::{start_element, end_element, attrs, control};

use fslib::conference::{conference_control};

pub fn serve<W: Write>(w: &mut EventWriter<W>) {
    start_element(w,
                  "configuration",
                  attrs(vec![
                      ("name","conference.conf"),
                      ("description", "Audio Conference")
                  ]));
    start_element(w, "caller-controls", None);
    control_groups(w);
    end_element(w);

    start_element(w, "profiles", None);
    end_element(w);

    end_element(w);
}

pub fn control_groups<W: Write>(w: &mut EventWriter<W>) {
    let groups = conference_control::groups();

    if let Err(_) = groups {
        return;
    }

    let groups = groups.unwrap();
    for g in groups {
        let details = conference_control::group_details(g.id).unwrap();
        start_element(w, "group", attrs(vec![("name", &g.name)]));
        for d in details {
            control(w, &d.action, &d.digits);
        }
        end_element(w);
    }
}
