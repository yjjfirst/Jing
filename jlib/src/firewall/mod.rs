pub mod iptables;

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
        .order_by(id)
        .load::<FirewallRule>(&mut conn)?;

    Ok(rows)
}

pub fn allow(ip: &str) -> Result<()> {
    set(ip, "allow")
}

pub fn deny(ip: &str) -> Result<()> {
    set(ip, "deny")
}

pub fn toggle(rule_id: i32) -> Result<()> {
    use crate::schema::firewall_rules::dsl::*;

    let mut conn = db_connect();
    let rule = firewall_rules
        .filter(id.eq(rule_id))
        .first::<FirewallRule>(&mut conn)?;

    let new_action = if rule.action == "allow" {
        "deny"
    } else {
        "allow"
    };

    if new_action == "allow" {
        iptables::allow_ip(&rule.ip_address);
    } else {
        iptables::deny_ip(&rule.ip_address);
    }

    set(&rule.ip_address, new_action)?;

    Ok(())
}

pub fn get_by_ip(ip: &str) -> Result<FirewallRule> {
    use crate::schema::firewall_rules::dsl::*;

    let mut conn = db_connect();
    let exists = firewall_rules
        .filter(ip_address.eq(ip))
        .first::<FirewallRule>(&mut conn)?;

    Ok(exists)
}

pub fn set(ip: &str, a: &str) -> Result<()> {
    use crate::schema::firewall_rules::dsl::*;

    let mut conn = db_connect();    
    match get_by_ip(ip) {
        Ok(_) => {
            diesel::update(firewall_rules.filter(ip_address.eq(ip)))
                .set((action.eq(a), created_at.eq(Local::now())))
                .execute(&mut conn)?;
        },
        Err(_) => {
            diesel::insert_into(firewall_rules)
                .values((ip_address.eq(ip), action.eq(a), created_at.eq(Local::now())))
                .load::<FirewallRule>(&mut conn)?;        
        }
    };

    Ok(())
}
