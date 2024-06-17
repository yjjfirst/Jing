use gloo_net::http::Request;
use std::collections::HashMap;
use yew::Properties;
use serde::{Serialize, Deserialize};

use super::BASE_URL;

#[derive(Clone, PartialEq, Deserialize, Properties, Serialize, Debug)]
pub struct UserContainer {
    pub user: User,
    pub vars: HashMap<String, Var>,
    pub params: HashMap<String, Param>
}

#[derive(Clone, PartialEq, Deserialize, Properties, Serialize, Debug)]
pub struct User {
    pub id: usize,
    pub domain_id: i32,
    pub user_id: String
}

#[derive(Clone, PartialEq, Deserialize, Properties, Serialize, Debug)]
pub struct Param {
    pub id: usize,
    pub user_id: usize,
    pub name: String,
    pub value: String
}

#[derive(Clone, PartialEq, Deserialize, Properties, Serialize, Debug)]
pub struct Var {
    pub id: usize,
    pub user_id: usize,
    pub name: String,
    pub value: String
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

    pub async fn get(domain: usize, id: i32) -> UserContainer {
        let endpoint = format!("{}/{}/user/{}", BASE_URL, domain, id);

        let response = Request::get(&endpoint).send().await.unwrap();
        let user: UserContainer = response.json().await.unwrap();

        user
    }

}
