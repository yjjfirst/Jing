use std::io::Write;
use xml::writer::{EventWriter, XmlEvent, events::StartElementBuilder};
use std::collections::HashMap;

pub fn start_element<W: Write>(w: &mut EventWriter<W>,tag: &str, attrs: Option<HashMap<&str, &str>>) {
    let mut builder: StartElementBuilder = XmlEvent::start_element(tag);
    match attrs {
        Some(attrs) => {
            for (key, value) in &attrs {
                builder = builder.attr(&key[..], &value[..]);
            }
        },
        None => {}
    }

    let event: XmlEvent = builder.into();
    w.write(event).unwrap();
}

pub fn end_element<W: Write>(w: &mut EventWriter<W>) {
    let e: XmlEvent = XmlEvent::end_element().into();
    w.write(e).unwrap();
}

pub fn param<W: Write>(w: &mut EventWriter<W>, name: &str, value: &str) {
    start_element(w, "param", Some(HashMap::from([("name", name), ("value", value)])));
    end_element(w);
}
