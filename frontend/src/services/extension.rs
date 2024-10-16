use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Extension {
    pub id: usize,
    pub exten: String,
    pub exten_type: String
}