use crate::schema::users;
use crate::util_macro::{Fields};
use crate::printable::{Printable};

#[derive(Identifiable, Queryable, Debug, PartialEq)]
#[derive(Fields)]
pub struct User {
    pub id: i32,
    pub domain_id: i32,
    pub user_id: String,
}

#[derive(Insertable)]
#[diesel(table_name=users)]
pub struct NewUser<'a> {
    pub domain_id: i32,
    pub user_id: &'a str,
}
