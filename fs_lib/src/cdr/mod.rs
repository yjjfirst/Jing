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

    Ok(vec![])
}

pub fn add_cdr<'a>(
    a_caller_id : &'a str,
    a_dest: &'a str,
    start_time: &'a str,
    duration: i32,
    uuid: &'a str
) -> Result<()>{
    let mut conn = db_connect();
    Ok(())
}
