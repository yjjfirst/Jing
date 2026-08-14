use diesel::prelude::*;
use serde::{Serialize,Deserialize};
use crate::error::{Result};
use crate::schema::gateway_param_helps;
use crate::db_connect;

#[derive(Identifiable,Queryable,Debug,Serialize,Deserialize, AsChangeset)]
#[derive(Clone,PartialEq)]
#[diesel(table_name = gateway_param_helps)]

pub struct GatewayParamHelp {
    pub id: i32,
    pub name: String,
    pub range_text: String,
    pub help_text: String
}

pub fn list() -> Result<Vec<GatewayParamHelp>> {
    use crate::schema::gateway_param_helps::dsl::*;
    let mut conn = db_connect();

    let results = gateway_param_helps
        .load::<GatewayParamHelp>(&mut conn)?;

    Ok(results)
}