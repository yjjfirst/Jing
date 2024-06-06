pub mod models;

use models::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::db_connect;
use crate::schema::cdr;
use crate::error::{Result};

pub fn all_cdrs() -> Result<Vec<Cdr>> {
    use crate::schema::cdr::dsl::*;

    let mut conn = db_connect();

    Ok(vec![])
}

pub fn add_cdr(
    caller_id_number: String,
    caller_id_name: String,
    destination_number: String,
    start_stamp: NaiveDateTime,
    answer_stamp: NaiveDateTime,
    end_stamp: NaiveDateTime,
    duration: i32,
    billsec: i32,
    hangup_cause: String,
) -> Result<()>{
    let mut conn = db_connect();
    let new_cdr = Cdr {
        caller_id_number,
        caller_id_name,
        destination_number,
        start_stamp,
        answer_stamp,
        end_stamp,
        duration,
        billsec,
        hangup_cause
    };

    diesel::insert_into(cdr::table)
        .values(&new_cdr)
        .execute(&mut conn)?;

    Ok(())
}
