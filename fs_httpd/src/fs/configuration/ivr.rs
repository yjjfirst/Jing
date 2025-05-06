extern crate fs_lib;

use xml::writer::{EventWriter};
use std::io::Write;
use fs_lib::ivr;
use fs_lib::sound_file;
use crate::fs::xml_utils::{start_element, end_element, attrs, entry};

pub fn serve<W: Write>(w: &mut EventWriter<W>) {
    start_element(w,
                  "configuration",
                  attrs(vec![
                      ("name","ivr.conf"),
                      ("description", "IVR Menus")
                  ]));
    start_element(w, "menus", None);
    let ivrs = ivr::list().unwrap();
    for i in ivrs {
        ivr(w, i);
    }

    end_element(w);
    end_element(w);
}

pub fn ivr<W: Write>(w: &mut EventWriter<W>, ivr: ivr::Ivr) {
    let mut ivr_attrs = ivr::ivr_attrs::list(ivr.id).unwrap();
    let mut vec_attrs: Vec<(&str, &str)> = Vec::new();

    vec_attrs.push(("name", &ivr.name));
    for a in &mut ivr_attrs {
        if a.name == "greet-long"
            || a.name == "greet-short"
            || a.name == "exit-sound"
            || a.name == "invalid-sound" {
            let sound_file_id = a.value.parse::<i32>().unwrap();
            let sound_file = sound_file::get(sound_file_id).unwrap();
            a.value = sound_file.name.clone();
        }

        vec_attrs.push((&a.name, &a.value));
    }

    start_element(w, "menu", attrs(vec_attrs));

    entries(w, ivr);
    end_element(w);
}

fn entries<W: Write>(w: &mut EventWriter<W>, ivr: ivr::Ivr) {
    let ivr_entries = ivr::ivr_entry::list(ivr.id).unwrap();
    for i in ivr_entries {
        entry(w, &i.digits, &i.dest_exten);
    }
}
