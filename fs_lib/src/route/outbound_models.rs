use crate::schema::outbound_routes;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize, AsChangeset)]
#[derive(Debug)]
pub struct OutboundRoute {
    pub id: i32,
    pub gateway_id: i32,
    pub priority: i32,
    pub condition: String
}

#[derive(Insertable)]
#[diesel(table_name=outbound_routes)]
pub struct NewOutboundRoute<'a>{
    pub gateway_id: i32,
    pub priority: i32,
    pub condition: &'a str
}
