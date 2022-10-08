use crate::schema::sound_files;

#[derive(Queryable, Debug)]
#[derive(Clone)]
pub struct SoundFile {
    pub id: i32,
    pub name: String,
    pub domain_id: i32,
    pub desc: Option<String>
}

#[derive(Insertable)]
#[diesel(table_name=sound_files)]
pub struct NewSoundFile<'a> {
    pub name: &'a str,
    pub domain_id: i32,
    #[diesel(column_name = description)]
    pub desc: Option<&'a str>,
}
