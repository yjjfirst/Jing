pub mod models;

use models::*;
use diesel::prelude::*;
use crate::db_connect;
use crate::schema::domains;
use crate::error::{Result, Error};
use crate::rt::{is_var, eval};

pub fn add_domain(domain_name: &str) -> Result<()>{
    let mut conn = db_connect();
    let new_domain = NewDomain {
        domain_name,
    };
    diesel::insert_into(domains::table)
        .values(&new_domain)
        .execute(&mut conn)?;

    Ok(())
}

pub fn del_domain(domain_id: i32) -> Result<()>{
    use crate::schema::domains::columns::id;

    let mut conn = db_connect();
    diesel::delete(domains::table)
        .filter(id.eq(domain_id))
        .execute(&mut conn)?;

    Ok(())
}

pub fn list_domains() -> Result<Vec<Domain>> {
    use crate::schema::domains::dsl::*;

    let mut conn = db_connect();
    let dms = domains
        .load::<Domain>(&mut conn)?;


    let dms = dms.into_iter().map(|d| {
        Domain {
            domain_name: if is_var(&d.domain_name) {
                eval(&d.domain_name)
            } else {
                d.domain_name
            },
            ..d
        }
    }).collect();

    Ok(dms)
}


pub fn get_domain_by_name(dn: String) -> Result<Domain> {
    use crate::schema::domains::dsl::*;

    let mut conn = db_connect();
    let dms = domains
        .load::<Domain>(&mut conn)?;

    let mut dms: Vec<Domain> = dms.into_iter().map(|d| {
        Domain {
            domain_name: if is_var(&d.domain_name) {
                eval(&d.domain_name)
            } else {
                d.domain_name
            },
            ..d
        }
    }).filter(|d| d.domain_name == dn).collect();

    if let Some(d) = dms.pop() {
        Ok(d)
    } else {
        Err(Error::Fslib("No such domain found".to_string()))
    }

}

pub fn get_domain(domain_id: i32) -> Result<Domain> {
    use crate::schema::domains::dsl::*;

    let mut conn = db_connect();
    let mut dms = domains
        .filter(id.eq(domain_id))
        .load::<Domain>(&mut conn)?;

    if let Some(d) = dms.pop() {
        Ok(d)
    } else {
        Err(Error::Fslib("No such domain found".to_string()))
    }
}
