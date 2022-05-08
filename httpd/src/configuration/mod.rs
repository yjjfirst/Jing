use std::io::BufWriter;
use xml::writer::{EmitterConfig};
use std::collections::HashMap;
use super::xml_utils::{start_element, end_element};

pub fn serve () -> tide::Result {
    let mut buf = BufWriter::new(Vec::new());
    let mut w = EmitterConfig::new().perform_indent(true).create_writer(&mut buf);

    start_element(&mut w, "document", HashMap::from([("type", "freeswitch/xml")]));
    start_element(&mut w, "sectiont", HashMap::from([("name", "configuration")]));
    start_element(&mut w, "configuration", HashMap::from([
        ("name", "sofia.conf"),
        ("description","sofia Endpoint")
    ]));

    end_element(&mut w);
    end_element(&mut w);
    end_element(&mut w);

    let response = buf.into_inner().unwrap();
    let response: String = String::from_utf8(response).unwrap();

    Ok(response.into())
}
