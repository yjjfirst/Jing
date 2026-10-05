use diesel::prelude::*;
use serde::{Serialize,Deserialize};
use crate::error::{Result};
use crate::schema::ivr_attr_helps;
use crate::db_connect;

#[derive(Identifiable,Queryable,Debug,Serialize,Deserialize, AsChangeset)]
#[derive(Clone,PartialEq)]
#[diesel(table_name = ivr_attr_helps)]

pub struct IvrAttrHelp {
    pub id: i32,
    pub name: String,
    pub range_text: String,
    pub help_text: String
}

pub fn list() -> Result<Vec<IvrAttrHelp>> {
    use crate::schema::ivr_attr_helps::dsl::*;
    let mut conn = db_connect();

    let results = ivr_attr_helps
        .order(id.asc())
        .load::<IvrAttrHelp>(&mut conn)?;

    Ok(results)
}