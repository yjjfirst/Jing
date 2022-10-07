pub mod outbound_models;
pub mod inbound_models;

use outbound_models::*;
use inbound_models::*;
use diesel::prelude::*;

use crate::db_connect;
use crate::schema::outbound_route;
use crate::schema::inbound_route;
use crate::error::{Error, Result};

pub fn all_outbounds() -> Result<Vec<OutboundRoute>>{
    use crate::schema::outbound_route::dsl::*;
    let mut conn = db_connect();

    let result = outbound_route
        .load::<OutboundRoute>(&mut conn)?;

    Ok(result)
}

pub fn add_outbound(gateway_id: i32, priority: i32, condition: &str) -> Result<()>{
    let mut conn = db_connect();

    let new_route = NewOutboundRoute {
        gateway_id,
        priority,
        condition
    };

    diesel::insert_into(outbound_route::table)
        .values(&new_route)
        .execute(&mut conn)?;

    Ok(())

}

pub fn del_outbound(outbound_id: i32) -> Result<()>{
    use crate::schema::outbound_route::columns::id;

    let mut conn = db_connect();
    diesel::delete(outbound_route::table)
        .filter(id.eq(outbound_id))
        .execute(&mut conn)?;

    Ok(())
}

pub fn all_inbound() -> Result<Vec<InboundRoute>>{
    use crate::schema::inbound_route::dsl::*;
    let mut conn = db_connect();

    let result = inbound_route
        .load::<InboundRoute>(&mut conn)?;

   Ok(result)
}

pub fn add_inboud(context: &str, condition: &str, dest_extension: &str) -> Result<()> {
    let exist = true;

    match exist {
        false => return Err(Error::Fslib("Route destination doesn't exist".to_string())),
        _ => (),
    }

    let mut conn = db_connect();
    let new_route = NewInboundRoute {
        context,
        condition,
        dest_extension
    };

    diesel::insert_into(inbound_route::table)
        .values(&new_route)
        .execute(&mut conn)?;

    Ok(())
}

pub fn del_inbound(inbound_id: i32) -> Result<()>{
    use crate::schema::inbound_route::columns::id;
    let mut conn = db_connect();

    diesel::delete(inbound_route::table)
        .filter(id.eq(inbound_id))
        .execute(&mut conn)?;

    Ok(())
}
