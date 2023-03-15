use diesel::prelude::*;
use crate::error::{Result};
use crate::db_connect;
use crate::schema::{agent_params};
use super::agent::{Agent};

#[derive(Identifiable,Queryable,Associations,Debug)]
#[derive(Clone,PartialEq)]
#[diesel(belongs_to(Agent))]
pub struct AgentParam {
    pub id: i32,
    pub agent_id: i32,
    pub name: String,
    pub value: String
}

#[derive(Insertable)]
#[diesel(table_name=agent_params)]
pub struct NewAgentParam<'a> {
    pub agent_id: i32,
    pub name: &'a str,
    pub value: &'a str
}

pub fn add_defaults(a_id: i32) -> Result<()> {
    let mut conn = db_connect();
    let params = vec![
        ("type", "callback"),
        ("status", "Available"),
        ("max-no-answer","3"),
        ("wrap-up-time", "10"),
        ("reject-delay-time","10"),
        ("busy-delay-time","60")
    ];

    for p in params {
        let new_param = NewAgentParam {
            agent_id: a_id,
            name: p.0,
            value: p.1
        };

        diesel::insert_into(agent_params::table)
            .values(&new_param)
            .execute(&mut conn)?;
    }

    Ok(())
}
