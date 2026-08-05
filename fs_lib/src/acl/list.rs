use serde::{Serialize, Deserialize};
use diesel::prelude::*;

use crate::db_connect;
use crate::error::{Error, Result};
use crate::rt;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct AclList {
    pub id: i32,
    pub acl_name: String,
    pub acl_default: String
}

pub fn list() -> Result<Vec<AclList>> {
    use crate::schema::acl_lists::dsl::*;

    let mut conn = db_connect();
    let lists = acl_lists
        .load::<AclList>(&mut conn)?;

    Ok(lists)
}

pub fn add(name: &str, default: &str) -> Result<i32> {
    use crate::schema::acl_lists::dsl::*;

    let mut conn = db_connect();
    let inserted: Vec<AclList> = diesel::insert_into(acl_lists)
        .values((acl_name.eq(name), acl_default.eq(default)))
        .load(&mut conn)?;

    if let Some(first) = inserted.first() {
        rt::reload_acl();
        Ok(first.id)
    } else {
        Err(Error::Fslib("Failed to insert acl_list".to_string()))
    }
}

pub fn del(list_id_arg: i32) -> Result<()> {
    use crate::schema::acl_lists::dsl::*;

    let mut conn = db_connect();

    diesel::delete(acl_lists.filter(id.eq(list_id_arg)))
        .execute(&mut conn)?;

    rt::reload_acl();    

    Ok(())
}

pub fn edit(list_id_arg: i32, new_name: &str, new_default: &str) -> Result<()> {
    use crate::schema::acl_lists::dsl::*;

    let mut conn = db_connect();

    diesel::update(acl_lists.filter(id.eq(list_id_arg)))
        .set((acl_name.eq(new_name), acl_default.eq(new_default)))
        .execute(&mut conn)?;

    rt::reload_acl();    

    Ok(())
}

pub fn get(list_id_arg: i32) -> Result<AclList> {
    use crate::schema::acl_lists::dsl::*;

    let mut conn = db_connect();

    let list = acl_lists.filter(id.eq(list_id_arg)).first::<AclList>(&mut conn)?;

    Ok(list)
}
