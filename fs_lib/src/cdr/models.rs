use crate::schema::cdr;
use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};

#[derive(Queryable, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryCdr {
    pub id: i32,
    pub caller_id_number: Option<String>,
    pub caller_id_name: Option<String>,
    pub destination_number: String,
    pub start_stamp: DateTime<Local>,
    pub answer_stamp: Option<DateTime<Local>>,
    pub end_stamp: DateTime<Local>,
    pub duration: i32,
    pub billsec: i32,
    pub hangup_cause: String,
}

#[derive(Insertable)]
#[diesel(table_name=cdr)]
pub struct Cdr {
    pub caller_id_number: String,
    pub caller_id_name: String,
    pub destination_number: String,
    pub start_stamp: DateTime<Local>,
    pub answer_stamp: Option<DateTime<Local>>,
    pub end_stamp: DateTime<Local>,
    pub duration: i32,
    pub billsec: i32,
    pub hangup_cause: String,
}
