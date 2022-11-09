use super::agent_param;

use diesel::prelude::*;
use crate::error::{Result,Error};
use crate::db_connect;
use crate::schema::{agents};
use crate::domain;
use crate::user;

#[derive(Identifiable,Queryable,Debug,PartialEq)]
#[derive(Clone)]
pub struct Agent {
    pub id: i32,
    pub domain_id: i32,
    pub user_id: i32,
    pub name: String,
    pub leg_timeout: i32,
}

#[derive(Insertable)]
#[diesel(table_name=agents)]
pub struct NewAgent<'a> {
    pub domain_id: i32,
    pub user_id: i32,
    pub name: &'a str,
    pub leg_timeout:i32,
}

pub fn add(domain_id: i32, user_id: i32, name: String, leg_timeout: i32) -> Result<()> {
    let mut conn = db_connect();

    let agent = NewAgent {
        domain_id: domain_id,
        user_id: user_id,
        name: &name,
        leg_timeout: leg_timeout,
    };

    let inserted = diesel::insert_into(agents::table)
        .values(&agent)
        .get_result::<Agent>(&mut conn)?;

    agent_param::add_defaults(inserted.id)?;

    Ok(())
}

pub fn del(a_id: i32) -> Result<()> {
    use crate::schema::agents::columns::id;
    let mut conn = db_connect();

    diesel::delete(agents::table)
        .filter(id.eq(a_id))
        .execute(&mut conn)?;

    Ok(())
}

pub fn all() -> Result<Vec<Agent>>{
    use crate::schema::agents::dsl::*;
    let mut conn = db_connect();

    let result = agents
        .load::<Agent>(&mut conn)?;

    Ok(result)
}

pub fn get(agent_id: i32) -> Result<Agent> {
    use crate::schema::agents::dsl::*;
    let mut conn = db_connect();

    let result = agents
        .find(agent_id)
        .first(&mut conn)?;

    Ok(result)

}

pub fn params(a_id: i32) -> Result<Vec<agent_param::AgentParam>> {
    use crate::schema::agents::dsl::*;
    let mut conn = db_connect();

    let agent = agents
        .find(a_id)
        .first::<Agent>(&mut conn)?;

    let params = agent_param::AgentParam::belonging_to(&agent)
        .load::<agent_param::AgentParam>(&mut conn)?;

    Ok(params)
}
