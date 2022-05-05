use std::io::Write;
use xml::writer::{EventWriter, XmlEvent, events::StartElementBuilder};
use std::collections::HashMap;

pub fn start_element<W: Write>(w: &mut EventWriter<W>,tag: &str, attrs: HashMap<&str, &str>) {
    let mut builder: StartElementBuilder = XmlEvent::start_element(tag);

    for (key, value) in &attrs {
        builder = builder.attr(&key[..], &value[..]);
    }

    let event: XmlEvent = builder.into();
    w.write(event).unwrap();
}

pub fn end_element<W: Write>(w: &mut EventWriter<W>) {
    let e: XmlEvent = XmlEvent::end_element().into();
    w.write(e).unwrap();
}
