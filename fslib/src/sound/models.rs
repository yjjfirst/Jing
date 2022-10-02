use crate::schema::sound;

#[derive(Queryable, Debug)]
#[derive(Clone)]
pub struct Sound {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub domain_id: i32,
    pub desc: Option<String>
}

#[derive(Insertable)]
#[table_name="sound"]
pub struct NewSound<'a> {
    pub name: &'a str,
    pub path: &'a str,
    pub domain_id: i32,
    #[column_name="description"]
    pub desc: Option<&'a str>,
}
