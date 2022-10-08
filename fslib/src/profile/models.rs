use crate::schema::{profiles};

#[derive(Queryable)]
#[derive(Debug)]
pub struct ProfileParam {
    pub id: i32,
    pub profile_id: i32,
    pub name: String,
    pub value: String
}

#[derive(Queryable, Identifiable, PartialEq, Debug)]
#[diesel(table_name=profiles)]
pub struct Profile {
    pub id: i32,
    pub name: String
}
