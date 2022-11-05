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
}

#[derive(Insertable)]
#[diesel(table_name=agents)]
pub struct NewAgent<'a> {
    pub domain_id: i32,
    pub user_id: i32,
    pub name: &'a str,
}

pub fn add(domain_id: i32, name: String, user_id: i32) -> Result<()> {
    let domain = domain::get_domain(domain_id)?;
    let user = user::get_user(user_id)?;

    let mut conn = db_connect();

    let agent = NewAgent {
        domain_id: domain_id,
        user_id: user_id,
        name: &name,
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
