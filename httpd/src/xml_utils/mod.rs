use std::io::Write;
use xml::writer::{EventWriter, XmlEvent, events::StartElementBuilder};

pub struct Attr<'a> {
    pub name: &'a str,
    pub value: &'a str
}

impl<'a> Attr<'a> {
    pub fn new(name: &'a str, value: &'a str) -> Self {
         Self {name: name, value: value}
    }
}

pub fn attrs<'a>(attrs: Vec<(&'a str, &'a str)>) -> Option<Vec<Attr>>{
    Some(attrs.iter().map(|attr| {
        Attr {
            name: attr.0,
            value: attr.1
        }
    }).collect::<Vec<Attr>>())
}

pub fn start_element<W: Write>(w: &mut EventWriter<W>,
                               tag: &str,
                               attrs: Option<Vec<Attr>>)
{
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
    start_element(w, "param", attrs(vec![("name", name), ("value", value)]));
    end_element(w);
}

pub fn variable<W: Write>(w: &mut EventWriter<W>, name: &str, value: &str) {
    start_element(w, "variable", attrs(vec![("name", name), ("value", value)]));
    end_element(w);

}

pub fn action<W: Write>(w: &mut EventWriter<W>, app: &str, data: &str) {
    start_element(w, "action", attrs(vec![("application", app), ("data", data)]));
    end_element(w);
}

pub fn control<W: Write>(w: &mut EventWriter<W>, action: &str, digits: &str) {
    start_element(w, "control", attrs(vec![("action", action), ("digits", digits)]));
    end_element(w);
}
