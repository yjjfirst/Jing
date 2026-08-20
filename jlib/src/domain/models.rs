use serde::Serialize;
use crate::schema::domains;

#[derive(Queryable, Debug, Serialize)]
#[derive(Clone)]
pub struct Domain {
    pub id: i32,
    pub domain_name: String,
}

#[derive(Insertable)]
#[diesel(table_name=domains)]
pub struct NewDomain<'a> {
    pub domain_name: &'a str,
}
