pub mod models;
pub mod gateway_param;
pub mod gateway_param_help;

use std::collections::HashMap;
use models::*;
use diesel::prelude::*;
use crate::db_connect;
use crate::schema::gateways;
use gateway_param::{GatewayParam};
use crate::error::{Result, Error};
use crate::rt;


pub fn add(profile_id: i32, gateway_name: String, params: HashMap<String,String>) -> Result<i32> {
    let mut conn = db_connect();
    let new_gateway = NewGateway {
        profile_id,
        gateway_name,
    };

    let inserted: Vec<Gateway> = diesel::insert_into(gateways::table)
        .values(&new_gateway)
        .load(&mut conn)?;

    for (key, value) in &params {
        GatewayParam::add(inserted.first().unwrap().id, key, value).unwrap();
    }

    rt::reload_mod("mod_sofia");
    Ok(inserted[0].id)
}

pub fn del(gateway_id: i32) -> Result<()>{
    use crate::schema::gateways::columns::id;

    let mut conn = db_connect();

    diesel::delete(gateways::table)
        .filter(id.eq(gateway_id))
        .execute(&mut conn)?;

    rt::reload_mod("mod_sofia");    
    Ok(())
}

pub fn list() -> Result<Vec<Gateway>> {
    use crate::schema::gateways::dsl::*;
    let mut conn = db_connect();

    let results = gateways
        .order_by(id.asc())
        .load::<Gateway>(&mut conn)?;

    Ok(results)
}

pub fn get(gateway_id: i32) -> Result<Gateway> {
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

pub fn update(g: &Gateway) -> Result<()> {
    let mut conn = db_connect();
    use crate::schema::gateways;
    use crate::schema::gateways::dsl::*;

    diesel::update(gateways::table)
        .filter(id.eq(g.id))
        .set(g)
        .execute(&mut conn)?;

    rt::reload_mod("mod_sofia");
    Ok(())
}

pub fn get_params(g_id: i32) -> Result<Vec<GatewayParam>> {
    let mut conn = db_connect();
    let gateway = get(g_id)?;

    let params = GatewayParam::belonging_to(&gateway)
        .load::<GatewayParam>(&mut conn)?;

    Ok(params)
}
