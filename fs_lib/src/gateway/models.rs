use crate::schema::{gateways};
use crate::profile::models::Profile;
use serde::{Serialize, Deserialize};

#[derive(Queryable,AsChangeset,Identifiable, Associations, PartialEq, Debug, Serialize, Deserialize)]
#[diesel(table_name = gateways)]
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
#[diesel(table_name=gateways)]
pub struct NewGateway {
    pub profile_id: i32,
    pub gateway_name: String,
    pub proxy: String,
    pub register: String,
    pub username: Option<String>,
    pub password: Option<String>
}
