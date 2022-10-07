use crate::schema::{gateway};
use crate::profile::models::Profile;

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[diesel(table_name = gateway)]
#[diesel(belongs_to(Profile))]
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
#[diesel(table_name=gateway)]
pub struct NewGateway {
    pub profile_id: i32,
    pub gateway_name: String,
    pub proxy: String,
    pub register: String,
    pub username: Option<String>,
    pub password: Option<String>
}
