use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::error::{Result};
use crate::db_connect;
use crate::schema::{agent_params};
use super::agent::{Agent};

#[derive(Identifiable,Queryable,Associations,Debug, AsChangeset)]
#[derive(Clone,PartialEq, Serialize, Deserialize)]
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

pub fn add(a_agent_id: i32, a_name: String, a_value: String ) -> Result<()>{
    use crate::schema::agent_params::dsl::*;
    use crate::schema::agent_params;

    let mut conn = db_connect();
    diesel::insert_into(agent_params::table)
        .values((&agent_id.eq(a_agent_id),
                 &name.eq(a_name),
                 &value.eq(a_value)))
        .execute(&mut conn)?;

    Ok(())
}

pub fn default_params() -> Vec<(&'static str, &'static str)> {
    vec![
        ("type", "callback"),
        ("status", "Available"),
        ("max-no-answer","3"),
        ("wrap-up-time", "10"),
        ("reject-delay-time","10"),
        ("busy-delay-time","60")
    ]
}

pub fn add_defaults(a_id: i32) -> Result<()> {
    let mut conn = db_connect();
    let params = default_params();

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

pub fn update(param: &AgentParam) -> Result<()> {
    use crate::schema::agent_params::dsl::*;
    use crate::schema::agent_params;
    let mut conn = db_connect();

    diesel::update(agent_params::table)
        .filter(id.eq(param.id))
        .set(param)
        .execute(&mut conn)?;

    Ok(())
}
