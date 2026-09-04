use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};
use diesel::prelude::*;

use crate::db_connect;
use crate::error::{Error, Result};

#[derive(Debug, Queryable, Serialize, Deserialize, Clone)]
#[diesel(table_name = firewall_rules)]
pub struct FirewallRule {
    pub id: i32,
    pub ip_address: String,
    pub action: String,
    pub created_at: DateTime<Local>,
}

pub fn list() -> Result<Vec<FirewallRule>> {
    use crate::schema::firewall_rules::dsl::*;

    let mut conn = db_connect();
    let rows = firewall_rules
        .load::<FirewallRule>(&mut conn)?;

    Ok(rows)
}

pub fn add(ip: &str, a: &str) -> Result<i32> {
    use crate::schema::firewall_rules::dsl::*;

    let mut conn = db_connect();
    let inserted: Vec<FirewallRule> = diesel::insert_into(firewall_rules)
        .values((ip_address.eq(ip), action.eq(a), created_at.eq(Local::now())))
        .load(&mut conn)?;

    if let Some(first) = inserted.first() {
        Ok(first.id)
    } else {
        Err(Error::Fslib("Failed to insert firewall entry".to_string()))
    }
}