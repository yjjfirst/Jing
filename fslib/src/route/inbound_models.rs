use crate::schema::inbound_route;

#[derive(Queryable)]
#[derive(Debug)]
pub struct InboundRoute {
    pub id: i32,
    pub context: String,
    pub condition: String,
    pub dest_type: String,
    pub dest: i32
}

#[derive(Insertable)]
#[table_name="inbound_route"]
pub struct NewInboundRoute<'a> {
    pub context: &'a str,
    pub condition: &'a str,
    pub dest_type: &'a str,
    pub dest: i32    
}
