use gloo_net::http::Request;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use web_sys::FormData;
use util_macro::HashMapHelper;


use super::BASE_URL;

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct UserAllData {
    pub user: User,
    pub vars: HashMap<String, Var>,
    pub params: HashMap<String, Param>
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct User {
    pub id: usize,
    pub domain_id: i32,
    pub user_id: String
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug, HashMapHelper)]
pub struct Param {
    pub id: usize,
    pub user_id: usize,
    pub name: String,
    pub value: String
}
impl Param {
    pub fn _new() -> Param {
        Param {id: 0, user_id: 0, name: "".to_string(), value: "".to_string()}
    }
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug, HashMapHelper)]
pub struct Var {
    pub id: usize,
    pub user_id: usize,
    pub name: String,
    pub value: String
}

impl Var {
    pub fn _new() -> Var {
        Var {id: 0, user_id: 0, name: "".to_string(), value: "".to_string()}
    }
}

impl User {
    pub fn new() -> User{
        User {id: 0, domain_id: 0, user_id: "".to_string()}
    }
    pub async fn list(domain: usize) -> Vec<String> {
        let endpoint = format!("{}/{}/user", BASE_URL, domain);
        let response = Request::get(&endpoint).send().await.unwrap();

        let extensions: Vec<User> = response.json().await.unwrap();

        extensions.iter().map(|e|{
            e.user_id.to_string()
        }).collect::<Vec<String>>()
    }
}
