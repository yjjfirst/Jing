use xml::writer::{EventWriter};
use std::io::Write;
use crate::fs::xml_utils::{start_element, end_element, Attr};

pub fn serve<W: Write>(w: &mut EventWriter<W>) {
    start_element(w, "configuration", Some(
        vec![Attr {name: "name", value: "acl.conf"},
             Attr {name: "description",value: "Network Lists"}]));
    start_element(w, "network-lists", None);
    start_element(w, "list", Some(vec![
        Attr {name: "name", value: "lan"},
        Attr {name: "default", value: "deny"}]));

    start_element(w, "node", Some(vec![
        Attr {name: "type", value: "allow"},
        Attr {name: "cidr", value: "192.168.1.1/32"}]));

    end_element(w);
    end_element(w);
    end_element(w);
    end_element(w);
}
