use crate::schema::outbound_route;

#[derive(Queryable)]
#[derive(Debug)]
pub struct OutboundRoute {
    pub id: i32,
    pub gateway_id: i32,
    pub priority: i32,
    pub condition: String
}

#[derive(Insertable)]
#[table_name="outbound_route"]
pub struct NewOutboundRoute<'a>{
    pub gateway_id: i32,
    pub priority: i32,
    pub condition: &'a str
}
