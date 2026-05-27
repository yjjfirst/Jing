use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use util_macro::HashMapHelper;
use web_sys::FormData;

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct Gateway {
    pub id: usize,
    pub gateway_name: String,
    pub profile_id: usize,
    pub params: HashMap<String, Param>
}

impl Gateway {
    pub fn new() -> Gateway {
        Gateway {
            id: 0,
            profile_id: 0,
            gateway_name: "".to_string(),
            params: HashMap::new()
       }
    }
    pub fn get_gateway_by_id(id: usize, gateways: &Vec<Gateway>) -> Option<&Gateway> {
        let pos = gateways
            .iter()
            .position(|g|{g.id == id})
            .unwrap();
        gateways.get(pos)
    }

    pub fn get_gateway_by_name(name: String, gateways: &Vec<Gateway>) -> Option<&Gateway> {
        let pos = gateways
            .iter()
            .position(|g|{g.gateway_name == name})
            .unwrap();
        gateways.get(pos)
    }
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug, HashMapHelper)]
pub struct Param {
    pub id: usize,
    pub gateway_id: usize,
    pub name: String,
    pub value: String
}

impl Param {
    pub fn _new() -> Param {
        Param {id: 0, gateway_id: 0, name: "".to_string(), value: "".to_string()}
    }
}
