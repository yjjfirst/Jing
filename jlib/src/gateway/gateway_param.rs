use diesel::prelude::*;
use serde::{Serialize,Deserialize};

use super::Gateway;
use crate::error::{Result};
use crate::schema::gateway_params::*;
use crate::schema::gateway_params;
use crate::schema::gateway_params::table;
use crate::util_macro::{Param, Fields};
use crate::db_connect;
use crate::printable::{Printable};

#[derive(Identifiable,Queryable,Associations,Debug,Serialize,Deserialize, AsChangeset)]
#[derive(Clone,PartialEq)]
#[derive(Param)]
#[derive(Fields)]
#[diesel(belongs_to(Gateway))]
pub struct GatewayParam {
    #[id]
    pub id: i32,
    #[parent_id]
    pub gateway_id: i32,
    #[name]
    pub name: String,
    #[value]
    pub value: String
}

pub fn add(g_id: i32, a_name: String, a_value: String) -> Result<()> {
    use crate::schema::gateway_params::dsl::*;
    use crate::schema::gateway_params;
    let mut conn = db_connect();

    diesel::insert_into(gateway_params::table)
        .values((&gateway_id.eq(g_id),
                &name.eq(a_name),
                &value.eq(a_value)))
        .execute(&mut conn)?;

    Ok(())

}

pub fn update(param: &GatewayParam) -> Result<()> {
    use crate::schema::gateway_params::dsl::*;
    use crate::schema::gateway_params;
    let mut conn = db_connect();

    diesel::update(gateway_params::table)
        .filter(id.eq(param.id))
        .set(param)
        .execute(&mut conn)?;

    Ok(())
}

pub fn default_params() -> Vec<(&'static str, &'static str)> {
    vec![
        ("proxy", ""),
        ("register", ""),
        ("username", ""),
        ("password", ""),
    ]
}
