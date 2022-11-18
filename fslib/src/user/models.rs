use crate::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub domain_id: i32,
    pub user_id: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name=users)]
pub struct NewUser<'a> {
    pub domain_id: i32,
    pub user_id: &'a str,
    pub password: &'a str,
}
