use std::io::Write;
use xml::writer::{EventWriter, XmlEvent, events::StartElementBuilder};

pub struct Attr<'a> {
    pub name: &'a str,
    pub value: &'a str
}

pub fn start_element<W: Write>(w: &mut EventWriter<W>,tag: &str, attrs: Option<Vec<Attr>>) {
    let mut builder: StartElementBuilder = XmlEvent::start_element(tag);
    match attrs {
        Some(attrs) => {
            for attr in &attrs {
                builder = builder.attr(attr.name, attr.value);
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
    start_element(w, "param", Some(vec![Attr { name: "name", value: name}, Attr {name: "value", value: value}]));
    end_element(w);
}
