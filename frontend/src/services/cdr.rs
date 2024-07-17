use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct Cdr {
    pub id: i32,
    pub caller_id_number: Option<String>,
    pub caller_id_name: Option<String>,
    pub destination_number: String,
    pub start_stamp: DateTime<Utc>,    
    pub answer_stamp: Option<DateTime<Utc>>,
    pub end_stamp: DateTime<Utc>,
    pub duration: i32,
    pub billsec: i32,
    pub hangup_cause: String,    
}