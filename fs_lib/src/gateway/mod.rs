pub mod models;

use models::*;
use diesel::prelude::*;
use crate::db_connect;
use crate::schema::gateways;
use crate::error::{Result, Error};

pub fn add_gateway (
    profile_id: i32,
    gateway_name: String,
    proxy: String,
    register: String,
    username: Option<String>,
    password: Option<String>) -> Result<()> {

    let mut conn = db_connect();

    let new_gateway = NewGateway {
        profile_id,
        gateway_name,
        proxy,
        register,
        username,
        password
    };

    diesel::insert_into(gateways::table)
        .values(&new_gateway)
        .execute(&mut conn)?;

    Ok(())
}

pub fn del_gateway(gateway_id: i32) -> Result<()>{
    use crate::schema::gateways::columns::id;

    let mut conn = db_connect();

    diesel::delete(gateways::table)
        .filter(id.eq(gateway_id))
        .execute(&mut conn)?;

    Ok(())
}

pub fn list() -> Result<Vec<Gateway>> {
    use crate::schema::gateways::dsl::*;
    let mut conn = db_connect();

    let results = gateways
        .load::<Gateway>(&mut conn)?;

    Ok(results)
}

pub fn get_gateway(gateway_id: i32) -> Result<Gateway> {
    use crate::schema::gateways::dsl::*;
    let mut conn = db_connect();

    let mut result = gateways
        .filter(id.eq(gateway_id))
        .load::<Gateway>(&mut conn)?;

    if let Some(g) = result.pop() {
        Ok(g)
    } else {
        Err(Error::Fslib("Gateway doesn't exist".to_string()))
    }
}
