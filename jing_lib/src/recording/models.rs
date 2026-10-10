use crate::schema::recordings;
use serde::{Serialize, Deserialize};

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, AsChangeset)]
#[derive(Clone)]
pub struct Recording {
    pub id: i32,
    pub name: String,
    pub domain_id: i32,
    pub description: Option<String>
}

#[derive(Insertable)]
#[diesel(table_name=recordings)]
pub struct NewRecording<'a> {
    pub name: &'a str,
    pub domain_id: i32,
    #[diesel(column_name = description)]
    pub description: Option<&'a str>,
}
