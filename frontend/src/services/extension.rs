use yew::Properties;
use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Deserialize, Properties, Serialize, Debug)]
pub struct Extension {
    pub id: usize,
    pub domain_id: i32,
    pub user_id: String
}