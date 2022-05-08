use crate::schema::dest_type;

#[derive(Queryable)]
#[derive(Debug)]
pub struct DestType {
    pub id: i32,
    pub dest_name: String;
}
