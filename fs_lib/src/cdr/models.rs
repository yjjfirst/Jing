use crate::schema::cdr;
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
