use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct Gateway {
    pub gateway_name: String,
    pub proxy: String,
    pub register: String,
    pub username: String,
    pub password: String,
}