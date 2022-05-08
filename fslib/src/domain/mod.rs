pub mod models;

use models::*;
use diesel::prelude::*;
use crate::db_connect;
use crate::schema::domain;
use crate::error::{Result};

pub fn add_domain(domain_name: &str) -> Result<()>{
    let conn = db_connect();
    let new_domain = NewDomain {
        domain_name
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


pub fn get_domain_name(db_id: i32) -> Result<String> {
    use crate::schema::domain::dsl::*;

    let conn = db_connect();
    let names = domain
        .select(domain_name)
        .filter(id.eq(db_id))
        .load::<String>(&conn)?;

    Ok(names[0].to_string())
}
