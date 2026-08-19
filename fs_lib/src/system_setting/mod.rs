use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use crate::schema::system_settings;
use super::db_connect;
use crate::error::{Result};

#[derive(Identifiable, AsChangeset, Queryable, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemSetting {
    pub id: i32,
    pub setting_section: String,
    pub setting_key: String,
    pub setting_value: String,
}

pub fn update(section: &str, key: &str, value: &str) -> Result<()> {
    use crate::schema::system_settings::dsl::*;
    use crate::schema::system_settings;

    let mut conn = db_connect();

    diesel::update(system_settings::table)
        .filter(setting_section.eq(section))
        .filter(setting_key.eq(key))
        .set(setting_value.eq(value))
        .execute(&mut conn)?;

    Ok(())
}

pub fn list() -> Result<Vec<SystemSetting>> {
    use crate::schema::system_settings::dsl::*;
    let mut conn = db_connect();

    let results = system_settings
        .order(id.asc())
        .load::<SystemSetting>(&mut conn)?;

    Ok(results)
}

pub fn get(section: &str, key: &str) -> Result<String> {
    use crate::schema::system_settings::dsl::*;
    let mut conn = db_connect();

    let result = system_settings
        .filter(setting_section.eq(section))
        .filter(setting_key.eq(key))
        .first::<SystemSetting>(&mut conn)?;

    Ok(result.setting_value.clone())
}