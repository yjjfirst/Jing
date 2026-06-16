use serde::{Serialize, Deserialize};
use diesel::prelude::*;

use crate::schema::acl_lists;
use crate::db_connect;
use crate::error::{Error, Result};

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct AclList {
    id: i32,
    acl_name: String,
    acl_default: String
}

pub fn list() -> Result<Vec<AclList>> {
    use crate::schema::acl_lists::dsl::*;

    let mut conn = db_connect();
    let lists = acl_lists
        .load::<AclList>(&mut conn)?;

    Ok(lists)
}
