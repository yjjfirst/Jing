use crate::schema::sounds;

#[derive(Identifiable,Queryable, Debug)]
#[derive(Clone)]
#[diesel(belongs_to(SoundFile))]
pub struct Sound {
    pub id: i32,
    pub exten: String,
    pub name: String,
    pub domain_id:i32,
    pub sound_file_id: i32
}

#[derive(Insertable)]
#[diesel(table_name=sounds)]
pub struct NewSound {
    pub exten: String,
    pub name: String,
    pub domain_id:i32,
    pub sound_file_id: i32
}
