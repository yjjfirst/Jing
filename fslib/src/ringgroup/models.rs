use crate::schema::ringing_group;

#[derive(Queryable)]
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
#[diesel(table_name=ringing_group)]
pub struct NewRinggroup<'a> {
    pub name: &'a str,
    pub group_id: &'a str,
    pub domain_id: i32,
    pub ring_time: Option<i32>,
    pub ring_strategy: Option<&'a str>,
}
