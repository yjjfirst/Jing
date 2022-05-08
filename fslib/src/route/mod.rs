pub mod outbound_models;
pub mod inbound_models;

use outbound_models::*;
use inbound_models::*;
use diesel::prelude::*;

use crate::db_connect;
use crate::schema::outbound_route;
use crate::schema::inbound_route;
use crate::error::{HornetError, Result};

pub fn all_outbounds() -> Result<Vec<OutboundRoute>>{
    use crate::schema::outbound_route::dsl::*;
    let conn = db_connect();

    let result = outbound_route
        .load::<OutboundRoute>(&conn)?;

    Ok(result)
}

pub fn add_outbound(gateway_id: i32, priority: i32, condition: &str) -> Result<()>{
    let conn = db_connect();

    let new_route = NewOutboundRoute {
        gateway_id,
        priority,
        condition
    };

    diesel::insert_into(outbound_route::table)
        .values(&new_route)
        .execute(&conn)?;

    Ok(())

}

pub fn del_outbound(outbound_id: i32) -> Result<()>{
    use crate::schema::outbound_route::columns::id;

    let conn = db_connect();
    diesel::delete(outbound_route::table)
        .filter(id.eq(outbound_id))
        .execute(&conn)?;

    Ok(())
}

pub fn all_inbound() -> Result<Vec<InboundRoute>>{
    use crate::schema::inbound_route::dsl::*;
    let conn = db_connect();

    let result = inbound_route
        .load::<InboundRoute>(&conn)?;

   Ok(result)
}

pub fn add_inboud(context: &str, condition: &str, dest_type: &str, dest: i32) -> Result<()> {
    let exist = dest_exists(dest_type)?;

    match exist {
        false => return Err(HornetError::DestNonExist),
        _ => (),
    }
    
    let conn = db_connect();
    let new_route = NewInboundRoute {
        context,
        condition,
        dest_type,
        dest
    };

    diesel::insert_into(inbound_route::table)
        .values(&new_route)
        .execute(&conn)?;
    
    Ok(())
}

pub fn del_inbound(inbound_id: i32) -> Result<()>{
    use crate::schema::inbound_route::columns::id;
    let conn = db_connect();

    diesel::delete(inbound_route::table)
        .filter(id.eq(inbound_id))
        .execute(&conn)?;

    Ok(())
}

pub fn dest_exists(dest: &str) -> Result<bool> {
    use crate::schema::dest_type::dsl::*;
    
    let conn = db_connect();

    let count = dest_type
        .filter(dest_name.eq(dest))
        .execute(&conn)?;
    
    Ok(count == 1)
}
