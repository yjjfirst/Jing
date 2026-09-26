use diesel::prelude::*;
use serde::{Serialize,Deserialize};
use crate::error::{Result};
use crate::schema::user_param_helps;
use crate::db_connect;

#[derive(Identifiable,Queryable,Debug,Serialize,Deserialize, AsChangeset)]
#[derive(Clone,PartialEq)]
#[diesel(table_name = user_param_helps)]

pub struct UserParamHelp {
    pub id: i32,
    pub name: String,
    pub range_text: String,
    pub help_text: String
}

pub fn list() -> Result<Vec<UserParamHelp>> {
    use crate::schema::user_param_helps::dsl::*;
    let mut conn = db_connect();

    let results = user_param_helps
        .load::<UserParamHelp>(&mut conn)?;

    Ok(results)
}