pub mod ivr_attrs;
pub mod ivr_entry;

use diesel::prelude::*;
use diesel::dsl::*;
use crate::db_connect;
use crate::error::{Result};
use super::extension::{add_extension, del_extension};
use serde::{Serialize, Deserialize};
use crate::schema::ivrs;

pub enum DestType {
    User(i32),
    Ringgroup(i32)
}

#[derive(Identifiable,Queryable,Debug,PartialEq,Serialize, Deserialize, AsChangeset)]
#[derive(Clone)]
pub struct Ivr {
    pub id: i32,
    pub exten: String,
    pub name: String,
    pub domain_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name=ivrs)]
pub struct NewIvr<'a> {
    pub exten: &'a str,
    pub name: &'a str,
    pub domain_id: i32,
}

pub fn add(name: &str, exten: &str, domain_id: i32, greet_long: &str, greet_short: &str) -> Result<()> {
    let mut conn = db_connect();
    let new_ivr = NewIvr {
        name, exten, domain_id
    };

    add_extension(new_ivr.exten, "ivr", new_ivr.domain_id)?;

    let inserted: Ivr = diesel::insert_into(ivrs::table)
        .values(&new_ivr)
        .get_result(&mut conn)?;

    ivr_attrs::add_defaults(inserted.id, greet_long, greet_short)?;

    Ok(())
}

pub fn ivr_exists(i: i32) -> Result<bool> {
    use crate::schema::ivrs::dsl::*;
    let  mut conn = db_connect();

    let result = select(exists(ivrs.filter(id.eq(i)))).get_result::<bool>(&mut conn)?;

    Ok(result)
}

pub fn del(i: i32) -> Result<()> {
    use crate::schema::ivrs::columns::id;

    let mut conn = db_connect();
    let Ivr {exten, ..} = get(i)?;

    diesel::delete(ivrs::table)
        .filter(id.eq(i))
        .execute(&mut conn)?;

    del_extension(&exten)?;
    Ok(())
}

pub fn list() -> Result<Vec<Ivr>> {
    use crate::schema::ivrs::dsl::*;

    let mut conn = db_connect();

    let results = ivrs
        .load::<Ivr>(&mut conn)?;

    Ok(results)
}

pub fn get(i: i32) -> Result<Ivr> {
    use crate::schema::ivrs::dsl::*;
    let mut conn = db_connect();

    let result = ivrs
        .find(i)
        .first(&mut conn)?;

    Ok(result)
}

pub fn update(ivr: Ivr) -> Result<()> {
    use crate::schema::ivrs;
    use crate::schema::ivrs::dsl::*;

    let mut conn = db_connect();
    diesel::update(ivrs::table)
        .filter(id.eq(ivr.id))
        .set(ivr)
        .execute(&mut conn)?;

    Ok(())
}

pub fn get_by(domain: i32, ext: &str) -> Result<Ivr> {
    use crate::schema::ivrs::dsl::*;
    let mut conn = db_connect();

    let result = ivrs
        .filter(domain_id.eq(domain))
        .filter(exten.eq(ext))
        .first(&mut conn)?;

    Ok(result)
}
