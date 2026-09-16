use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct SystemSetting {
    pub id: i32,
    pub setting_section: String,
    pub setting_key: String,
    pub setting_value: String,
}