use crate::schema::cdr;
use chrono;

#[derive(Insertable)]
#[diesel(table_name=cdr)]
pub struct Cdr {
    pub caller_id_number: String,
    pub caller_id_name: String,
    pub destination_number: String,
    pub start_stamp: chrono::NaiveDateTime,
    pub answer_stamp: Option<chrono::NaiveDateTime>,
    pub end_stamp: chrono::NaiveDateTime,
    pub duration: i32,
    pub billsec: i32,
    pub hangup_cause: String,
}
