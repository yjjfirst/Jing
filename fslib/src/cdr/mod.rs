pub mod models;

use models::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::db_connect;
use crate::schema::cdr;
use crate::error::{Result, Error};

pub fn all_cdrs() -> Result<Vec<Cdr>> {
    use crate::schema::cdr::dsl::*;

    let mut conn = db_connect();

    let results = cdr
        .load::<Cdr>(&mut conn)?;

    Ok(results)
}

pub fn add_cdr<'a>(
    a_caller_id : &'a str,
    a_dest: &'a str,
    start_time: &'a str,
    duration: i32,
    uuid: &'a str
) -> Result<()>{
    let mut conn = db_connect();
    let new_cdr = NewCdr {
        a_caller_id,
        a_dest,
        start_time: &NaiveDateTime::from_timestamp(start_time.parse::<i64>().unwrap(), 0),
        duration,
        uuid,
    };

    diesel::insert_into(cdr::table)
        .values(&new_cdr)
        .execute(&mut conn)?;

    Ok(())
}

pub fn add_bleg<'a> (
    caller_id : &'a str,
    dest: &'a str,
    a_uuid: &'a str
) -> Result<()> {
    use crate::schema::cdr::dsl::*;

    let mut conn = db_connect();


    let count = cdr
        .filter(uuid.eq(a_uuid))
        .execute(&mut conn)?;

    if count == 1  {
        diesel::update(cdr.filter(uuid.eq(a_uuid)))
            .set(b_caller_id.eq(caller_id))
            .execute(&mut conn)?;

        diesel::update(cdr.filter(uuid.eq(a_uuid)))
            .set(b_dest.eq(dest))
            .execute(&mut conn)?;

        Ok(())

    } else {
       Err(Error::Fslib("Aleg doesn't exist".to_string()))
    }
}
