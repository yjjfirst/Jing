use serde::{Serialize, Deserialize};
use crate::schema::announcements;

#[derive(Identifiable,Queryable, Debug, Serialize, Deserialize, AsChangeset)]
#[derive(Clone)]
#[diesel(belongs_to(Recording))]
pub struct Announcement {
    pub id: i32,
    pub exten: String,
    pub name: String,
    pub domain_id:i32,
    pub recording_id: i32
}

#[derive(Insertable)]
#[diesel(table_name=announcements)]
pub struct NewAnnouncement {
    pub exten: String,
    pub name: String,
    pub domain_id:i32,
    pub recording_id: i32
}
