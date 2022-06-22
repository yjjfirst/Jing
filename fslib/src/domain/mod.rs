pub mod models;

use models::*;
use diesel::prelude::*;
use crate::db_connect;
use crate::schema::domain;
use crate::error::{Result, Error};

pub fn add_domain(domain_name: &str, active: bool) -> Result<()>{
    let conn = db_connect();
    let new_domain = NewDomain {
        domain_name,
        active
    };
    diesel::insert_into(domain::table)
        .values(&new_domain)
        .execute(&conn)?;

    Ok(())
}

pub fn del_domain(domain_id: i32) -> Result<()>{
    use crate::schema::domain::columns::id;

    let conn = db_connect();
    diesel::delete(domain::table)
        .filter(id.eq(domain_id))
        .execute(&conn)?;

    Ok(())
}

pub fn list_domains() -> Result<Vec<Domain>> {
    use crate::schema::domain::dsl::*;

    let conn = db_connect();
    let result = domain
        .load::<Domain>(&conn)?;

    Ok(result)
}


pub fn get_domain(domain_id: i32) -> Result<Domain> {
    use crate::schema::domain::dsl::*;

    let conn = db_connect();
    let mut domains = domain
        .filter(id.eq(domain_id))
        .load::<Domain>(&conn)?;

    if let Some(d) = domains.pop() {
        Ok(d)
    } else {
        Err(Error::Fslib("No active domain found".to_string()))
    }
}

pub fn set_active(domain_id: i32) -> Result<()> {
    use crate::schema::domain::dsl::*;
    let conn = db_connect();

    diesel::update(domain.filter(active.eq(true)))
        .set(active.eq(false))
        .execute(&conn)?;

    diesel::update(domain.filter(id.eq(domain_id)))
        .set(active.eq(true))
        .execute(&conn)?;

    Ok(())
}

pub fn get_active() -> Result<Domain> {
    use crate::schema::domain::dsl::*;

    let conn = db_connect();
    let mut domains = domain
        .filter(active.eq(true))
        .load::<Domain>(&conn)?;

    if let Some(d) = domains.pop() {
        Ok(d)
    } else {
        Err(Error::Fslib("No active domain found".to_string()))
    }
}
