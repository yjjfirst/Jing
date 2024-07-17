use super::inbound_models::*;
use diesel::prelude::*;

use crate::db_connect;
use crate::schema::inbound_routes;
use crate::error::{Error, Result};

pub fn list() -> Result<Vec<InboundRoute>>{
    use crate::schema::inbound_routes::dsl::*;
    let mut conn = db_connect();

    let result = inbound_routes
        .load::<InboundRoute>(&mut conn)?;

   Ok(result)
}

pub fn add(context: &str, condition: &str, dest_extension: &str) -> Result<()> {
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

    diesel::insert_into(inbound_routes::table)
        .values(&new_route)
        .execute(&mut conn)?;

    Ok(())
}

pub fn del(inbound_id: i32) -> Result<()>{
    use crate::schema::inbound_routes::columns::id;
    let mut conn = db_connect();

    diesel::delete(inbound_routes::table)
        .filter(id.eq(inbound_id))
        .execute(&mut conn)?;

    Ok(())
}

pub fn get(in_id: i32) -> Result<InboundRoute> {
    use crate::schema::inbound_routes::dsl::*;
    let mut conn = db_connect();

    let result = inbound_routes
        .find(in_id)
        .first::<InboundRoute>(&mut conn)?;

    Ok(result)
}

pub fn update(r: &InboundRoute) -> Result<()> {
    let mut conn = db_connect();
    use crate::schema::inbound_routes;
    use crate::schema::inbound_routes::dsl::*;

    diesel::update(inbound_routes::table)
        .filter(id.eq(r.id))
        .set(r)
        .execute(&mut conn)?;

    Ok(())
}
