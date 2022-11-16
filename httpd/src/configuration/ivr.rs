extern crate fslib;

use xml::writer::{EventWriter};
use std::io::Write;
use fslib::ivr;
use crate::xml_utils::{start_element, end_element, attrs};

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

    for a in &ivr_attrs {
        vec_attrs.push((&a.name, &a.value));
    }

    start_element(w, "menu", attrs(vec_attrs));

    entries(w, ivr);
    end_element(w);
}

fn entries<W: Write>(_w: &mut EventWriter<W>, ivr: ivr::Ivr) {
    let _ivr_entries = ivr::entries(ivr.id);
}
