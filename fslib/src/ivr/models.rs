use crate::schema::ivrs;

#[derive(Queryable)]
pub struct Ivr {
    pub id: i32,
    pub exten: String,
    pub name: String,
    pub domain_id: i32,
    pub greet_long: Option<String>,
    pub greet_short: Option<String>,
    pub invalid_sound: Option<String>,
    pub exit_sound: Option<String>,
    pub confirm_attempts: Option<i32>,
    pub timeout: Option<i32>,
    pub inter_digit_timeout: Option<i32>,
    pub max_failures: Option<i32>,
    pub max_timeouts: Option<i32>,
    pub digit_len: Option<i32>
}

#[derive(Insertable)]
#[diesel(table_name=ivrs)]
pub struct NewIvr<'a> {
    pub exten: &'a str,
    pub name: &'a str,
    pub domain_id: i32,
    pub greet_long: Option<&'a str>,
    pub greet_short: Option<&'a str>,
    pub invalid_sound: Option<&'a str>,
    pub exit_sound: Option<&'a str>,
}
