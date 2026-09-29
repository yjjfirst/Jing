use xml::writer::{EventWriter};
use std::io::Write;
use crate::fs::xml_utils::{start_element, end_element, Attr};

use jlib::acl::list::{AclList, list};
use jlib::acl::node::{list_by};

pub fn serve<W: Write>(w: &mut EventWriter<W>) {
    let lists = list().unwrap();
    start_element(w, "configuration", Some(
        vec![Attr {name: "name", value: "acl.conf"},
             Attr {name: "description",value: "Network Lists"}]));
    start_element(w, "network-lists", None);
    
    for list in lists {
        serve_list(w, &list);
    }

    end_element(w);
    end_element(w);
}

pub fn serve_list<W: Write>(w: &mut EventWriter<W>, list: &AclList) {
    start_element(w, "list", Some(vec![
        Attr {name: "name", value: list.acl_name.as_str()},
        Attr {name: "default", value: list.acl_default.as_str()}]));
    serve_nodes(w, list.id);
    end_element(w);
}

pub fn serve_nodes<W: Write>(w: &mut EventWriter<W>, list_id: i32) {
    
    let nodes = list_by(Some(list_id)).unwrap();

    for node in nodes {
        start_element(w, "node", Some(vec![
            Attr {name: "type", value: node.node_type.as_str()},
            Attr {name: "cidr", value: node.cidr.as_str()}]));

        end_element(w);
    }
}
