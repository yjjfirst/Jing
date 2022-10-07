pub mod models;

use models::*;
use diesel::prelude::*;
use crate::db_connect;
use crate::schema::domain;
use crate::error::{Result, Error};
use crate::rt::{is_var, eval};

pub fn add_domain(domain_name: &str, active: bool) -> Result<()>{
    let mut conn = db_connect();
    let new_domain = NewDomain {
        domain_name,
        active
    };
    diesel::insert_into(domain::table)
        .values(&new_domain)
        .execute(&mut conn)?;

    Ok(())
}

pub fn del_domain(domain_id: i32) -> Result<()>{
    use crate::schema::domain::columns::id;

    let mut conn = db_connect();
    diesel::delete(domain::table)
        .filter(id.eq(domain_id))
        .execute(&mut conn)?;

    Ok(())
}

pub fn list_domains() -> Result<Vec<Domain>> {
    use crate::schema::domain::dsl::*;

    let mut conn = db_connect();
    let result = domain
        .load::<Domain>(&mut conn)?;

    Ok(result)
}


pub fn get_domain_by_name(dn: String) -> Result<Domain> {
    use crate::schema::domain::dsl::*;

    let mut conn = db_connect();
    let domains = domain
        .load::<Domain>(&mut conn)?;

    let mut domains: Vec<Domain> = domains.into_iter().map(|d| {
        Domain {
            domain_name: if is_var(&d.domain_name) {
                eval(&d.domain_name)
            } else {
                d.domain_name
            },
            ..d
        }
    }).filter(|d| d.domain_name == dn).collect();

    if let Some(d) = domains.pop() {
        Ok(d)
    } else {
        Err(Error::Fslib("No such domain found".to_string()))
    }

}

pub fn get_domain(domain_id: i32) -> Result<Domain> {
    use crate::schema::domain::dsl::*;

    let mut conn = db_connect();
    let mut domains = domain
        .filter(id.eq(domain_id))
        .load::<Domain>(&mut conn)?;

    if let Some(d) = domains.pop() {
        Ok(d)
    } else {
        Err(Error::Fslib("No such domain found".to_string()))
    }
}

pub fn set_active(domain_id: i32) -> Result<()> {
    use crate::schema::domain::dsl::*;
    let mut conn = db_connect();

    diesel::update(domain.filter(active.eq(true)))
        .set(active.eq(false))
        .execute(&mut conn)?;

    diesel::update(domain.filter(id.eq(domain_id)))
        .set(active.eq(true))
        .execute(&mut conn)?;

    Ok(())
}

pub fn get_active() -> Result<Domain> {
    use crate::schema::domain::dsl::*;

    let mut conn = db_connect();
    let mut domains = domain
        .filter(active.eq(true))
        .load::<Domain>(&mut conn)?;

    if let Some(d) = domains.pop() {
        Ok(d)
    } else {
        Err(Error::Fslib("No active domain found".to_string()))
    }
}
