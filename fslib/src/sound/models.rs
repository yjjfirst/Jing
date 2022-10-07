use crate::schema::sound;

#[derive(Queryable, Debug)]
#[derive(Clone)]
pub struct Sound {
    pub id: i32,
    pub exten: String,
    pub name: String,
    pub domain_id:i32,
    pub sound_file_id: i32
}

#[derive(Insertable)]
#[diesel(table_name=sound)]
pub struct NewSound {
    pub exten: String,
    pub name: String,
    pub domain_id:i32,
    pub sound_file_id: i32
}
