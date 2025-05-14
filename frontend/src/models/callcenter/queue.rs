use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]

pub struct Queue {
    pub id: usize,
    pub domain_id: usize,
    pub exten: String,
    pub name: String,
}