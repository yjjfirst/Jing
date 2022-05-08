use crate::schema::gateway;

#[derive(Queryable)]
pub struct Gateway {
    pub id: i32,
    pub profile_id: i32,
    pub gateway_name: String,
    pub proxy: String,
    pub register: String,
    pub username: Option<String>,
    pub password: Option<String>
}

#[derive(Insertable)]
#[table_name="gateway"]
pub struct NewGateway {
    pub profile_id: i32,
    pub gateway_name: String,
    pub proxy: String,
    pub register: String,
    pub username: Option<String>,
    pub password: Option<String>
}
