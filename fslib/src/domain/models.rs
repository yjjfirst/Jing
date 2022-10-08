use crate::schema::domains;

#[derive(Queryable, Debug)]
#[derive(Clone)]
pub struct Domain {
    pub id: i32,
    pub domain_name: String,
    pub active: bool,
}

#[derive(Insertable)]
#[diesel(table_name=domains)]
pub struct NewDomain<'a> {
    pub domain_name: &'a str,
    pub active: bool,
}
