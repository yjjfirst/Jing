use diesel::prelude::*;
use serde::{Serialize,Deserialize};
use crate::error::{Result};
use crate::schema::agent_param_helps;
use crate::db_connect;

#[derive(Identifiable,Queryable,Debug,Serialize,Deserialize, AsChangeset)]
#[derive(Clone,PartialEq)]
#[diesel(table_name = agent_param_helps)]
pub struct AgentParamHelp {
    pub id: i32,
    pub name: String,
    pub range_text: String,
    pub help_text: String
}

pub fn list() -> Result<Vec<AgentParamHelp>> {
    use crate::schema::agent_param_helps::dsl::*;
    let mut conn = db_connect();

    let results = agent_param_helps
        .order_by(id.asc())
        .load::<AgentParamHelp>(&mut conn)?;

    Ok(results)
}