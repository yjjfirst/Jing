pub mod models;

use models::*;
use diesel::prelude::*;
use crate::db_connect;
use crate::schema::gateway;
use crate::error::{Result};

pub fn add_gateway (
    profile_id: i32,
    gateway_name: String,
    proxy: String,
    register: String,
    username: Option<String>,
    password: Option<String>) -> Result<()> {

    let conn = db_connect();

    let new_gateway = NewGateway {
        profile_id,
        gateway_name,
        proxy,
        register,
        username,
        password
    };

    diesel::insert_into(gateway::table)
        .values(&new_gateway)
        .execute(&conn)?;

    Ok(())
}

pub fn del_gateway(gateway_id: i32) -> Result<()>{
    use crate::schema::gateway::columns::id;

    let conn = db_connect();

    diesel::delete(gateway::table)
        .filter(id.eq(gateway_id))
        .execute(&conn)?;

    Ok(())
}

pub fn all_gateways() -> Result<Vec<Gateway>> {
    use crate::schema::gateway::dsl::*;
    let conn = db_connect();

    let results = gateway
        .load::<Gateway>(&conn)?;

    Ok(results)
}
