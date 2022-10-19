use xml::writer::{EventWriter};
use std::io::Write;
use crate::xml_utils::{start_element, end_element, attrs, control, param};

pub fn serve<W: Write>(w: &mut EventWriter<W>) {
    start_element(w,
                  "configuration",
                  attrs(vec![
                      ("name","callcenter.conf"),
                      ("description", "CallCenter")
                  ]));
    start_element(w, "settings", None);


    end_element(w);
    end_element(w);
}
