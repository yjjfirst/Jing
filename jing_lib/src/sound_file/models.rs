use crate::schema::sound_files;
use serde::{Serialize, Deserialize};

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, AsChangeset)]
#[derive(Clone)]
pub struct SoundFile {
    pub id: i32,
    pub name: String,
    pub domain_id: i32,
    pub description: Option<String>
}

#[derive(Insertable)]
#[diesel(table_name=sound_files)]
pub struct NewSoundFile<'a> {
    pub name: &'a str,
    pub domain_id: i32,
    #[diesel(column_name = description)]
    pub description: Option<&'a str>,
}
