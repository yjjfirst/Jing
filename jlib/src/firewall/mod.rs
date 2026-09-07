use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};
use diesel::prelude::*;

use crate::db_connect;
use crate::error::{Result};

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

pub fn allow(ip: &str) -> Result<()> {
    set(ip, "allow")
}

pub fn deny(ip: &str) -> Result<()> {
    set(ip, "deny")
}

pub fn exists(ip: &str) -> Result<bool> {
    use crate::schema::firewall_rules::dsl::*;

    let mut conn = db_connect();
    let exists = firewall_rules
        .filter(ip_address.eq(ip))
        .first::<FirewallRule>(&mut conn)
        .optional()?;

    Ok(exists.is_some())
}

pub fn set(ip: &str, a: &str) -> Result<()> {
    use crate::schema::firewall_rules::dsl::*;

    let mut conn = db_connect();

    let exists = firewall_rules
        .filter(ip_address.eq(ip))
        .first::<FirewallRule>(&mut conn)
        .optional()?;
    
    match exists {
        Some(_) => {
            diesel::update(firewall_rules.filter(ip_address.eq(ip)))
                .set((action.eq(a), created_at.eq(Local::now())))
                .execute(&mut conn)?;
        },
        None => {
            diesel::insert_into(firewall_rules)
                .values((ip_address.eq(ip), action.eq(a), created_at.eq(Local::now())))
                .load::<FirewallRule>(&mut conn)?;        
        }
    };

    Ok(())
}
