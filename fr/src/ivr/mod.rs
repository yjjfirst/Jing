pub mod models;

use models::*;
use diesel::prelude::*;
use crate::db_connect;
use crate::error::{Result};

pub fn add(new_ivr: NewIvr) -> Result<()> {
    use crate::schema::ivr;
    let conn = db_connect();

    diesel::insert_into(ivr::table)
        .values(&new_ivr)
        .execute(&conn)?;
    
    Ok(())
}

pub fn del(i: i32) -> Result<()> {
    use crate::schema::ivr;
    use crate::schema::ivr::columns::id;

    let conn = db_connect();

    diesel::delete(ivr::table)
        .filter(id.eq(i))
        .execute(&conn)?;

    Ok(())
}

pub fn all() -> Result<Vec<Ivr>> {
    use crate::schema::ivr::dsl::*;

    let conn = db_connect();

    let results = ivr
        .load::<Ivr>(&conn)?;

    Ok(results)
}

pub fn add_ivr_option(digits: String, dest_type: i32, dest_id: i32) {
}

pub fn del_ivr_option(i: i32) {
}
