extern crate fs_lib;

use xml::writer::{EventWriter};
use std::io::Write;
use fs_lib::ivr;
use crate::xml_utils::{start_element, end_element, attrs, entry};

pub fn serve<W: Write>(w: &mut EventWriter<W>) {
    start_element(w,
                  "configuration",
                  attrs(vec![
                      ("name","ivr.conf"),
                      ("description", "IVR Menus")
                  ]));
    start_element(w, "menus", None);
    let ivrs = ivr::all().unwrap();
    for i in ivrs {
        ivr(w, i);
    }

    end_element(w);
    end_element(w);
}

pub fn ivr<W: Write>(w: &mut EventWriter<W>, ivr: ivr::Ivr) {
    let ivr_attrs = ivr::attrs(ivr.id).unwrap();
    let mut vec_attrs: Vec<(&str, &str)> = Vec::new();

    vec_attrs.push(("name", &ivr.name));
    for a in &ivr_attrs {
        vec_attrs.push((&a.name, &a.value));
    }

    start_element(w, "menu", attrs(vec_attrs));

    entries(w, ivr);
    end_element(w);
}

fn entries<W: Write>(w: &mut EventWriter<W>, ivr: ivr::Ivr) {
    let ivr_entries = ivr::entries(ivr.id).unwrap();
    for i in ivr_entries {
        entry(w, &i.digits, &i.dest_exten);
    }
}
