use crate::schema::cdrs;
use chrono;

#[derive(Queryable)]
pub struct Cdr {
    pub id: i32,
    pub a_caller_id: String,
    pub a_dest: String,
    pub start_time: chrono::NaiveDateTime,
    pub duration: i32,
    pub b_caller_id: Option<String>,
    pub b_dest: Option<String>,
    pub uuid: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name=cdrs)]
pub struct NewCdr<'a> {
    pub a_caller_id: &'a str,
    pub a_dest: &'a str,
    pub start_time: &'a chrono::NaiveDateTime,
    pub duration: i32,
    pub uuid: &'a str,
}
