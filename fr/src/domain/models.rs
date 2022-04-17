use crate::schema::domain;

#[derive(Queryable)]
pub struct Domain {
    pub id: i32,
    pub domain_name: String
}

#[derive(Insertable)]
#[table_name="domain"]
pub struct NewDomain<'a> {
    pub domain_name: &'a str        
}
