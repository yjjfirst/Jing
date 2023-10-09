use crate::schema::ringing_groups;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize, Debug, AsChangeset)]
#[diesel(table_name=ringing_groups)]
#[diesel(primary_key(id, group_id))]
pub struct Ringgroup {
    pub id: i32,
    pub name: String,
    pub group_id: String,
    pub domain_id: i32,
    pub description: Option<String>,
    pub ring_time: i32,
    pub ring_strategy: String,
}

#[derive(Insertable)]
#[diesel(table_name=ringing_groups)]
pub struct NewRinggroup<'a> {
    pub name: &'a str,
    pub group_id: &'a str,
    pub domain_id: i32,
    pub ring_time: Option<i32>,
    pub ring_strategy: Option<&'a str>,
}
