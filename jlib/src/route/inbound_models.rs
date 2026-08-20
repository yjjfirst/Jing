use crate::schema::inbound_routes;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize, AsChangeset)]
#[derive(Debug)]
pub struct InboundRoute {
    pub id: i32,
    pub context: String,
    pub condition: String,
    pub dest_extension: String,
}

#[derive(Insertable)]
#[diesel(table_name=inbound_routes)]
pub struct NewInboundRoute<'a> {
    pub context: &'a str,
    pub condition: &'a str,
    pub dest_extension: &'a str,
}
