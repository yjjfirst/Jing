
use super::outbound_models::*;
use diesel::prelude::*;

use crate::db_connect;
use crate::schema::outbound_routes;
use crate::error::{Result};

pub fn list() -> Result<Vec<OutboundRoute>>{
    use crate::schema::outbound_routes::dsl::*;
    let mut conn = db_connect();

    let result = outbound_routes
        .load::<OutboundRoute>(&mut conn)?;

    Ok(result)
}

pub fn add(gateway_id: i32, priority: i32, condition: &str) -> Result<()>{
    let mut conn = db_connect();

    let new_route = NewOutboundRoute {
        gateway_id,
        priority,
        condition
    };

    diesel::insert_into(outbound_routes::table)
        .values(&new_route)
        .execute(&mut conn)?;

    Ok(())

}

pub fn del(outbound_id: i32) -> Result<()>{
    use crate::schema::outbound_routes::columns::id;

    let mut conn = db_connect();
    diesel::delete(outbound_routes::table)
        .filter(id.eq(outbound_id))
        .execute(&mut conn)?;

    Ok(())
}

pub fn get(out_id: i32) -> Result<OutboundRoute> {
    use crate::schema::outbound_routes::dsl::*;
    let mut conn = db_connect();

    let result = outbound_routes
        .find(out_id)
        .first::<OutboundRoute>(&mut conn)?;

    Ok(result)
}

pub fn update(r: &OutboundRoute) -> Result<()> {
    let mut conn = db_connect();
    use crate::schema::outbound_routes;
    use crate::schema::outbound_routes::dsl::*;

    diesel::update(outbound_routes::table)
        .filter(id.eq(r.id))
        .set(r)
        .execute(&mut conn)?;

    Ok(())
}
