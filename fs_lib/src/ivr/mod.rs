pub mod ivr_attrs;
pub mod ivr_entry;

use diesel::prelude::*;
use diesel::dsl::*;
use crate::db_connect;
use crate::extension;
use crate::error::{Result, Error};
use super::extension::{add_extension, del_extension};
use serde::{Serialize, Deserialize};
use crate::schema::ivrs;

use ivr_attrs::{IvrAttr};
use ivr_entry::{IvrEntry};
pub enum DestType {
    User(i32),
    Ringgroup(i32)
}

#[derive(Identifiable,Queryable,Debug,PartialEq,Serialize, Deserialize)]
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

pub fn add(name: &str, exten: &str, domain_id: i32) -> Result<()> {
    let mut conn = db_connect();
    let new_ivr = NewIvr {
        name, exten, domain_id
    };

    add_extension(new_ivr.exten, "ivr", new_ivr.domain_id)?;

    let inserted: Ivr = diesel::insert_into(ivrs::table)
        .values(&new_ivr)
        .get_result(&mut conn)?;

    ivr_attrs::add_defaults(inserted.id)?;

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

pub fn get_by(domain: i32, ext: &str) -> Result<Ivr> {
    use crate::schema::ivrs::dsl::*;
    let mut conn = db_connect();

    let result = ivrs
        .filter(domain_id.eq(domain))
        .filter(exten.eq(ext))
        .first(&mut conn)?;

    Ok(result)
}

pub fn add_ivr_attr(a_ivr_id: i32, a_name: String, a_value: String) -> Result<()> {
    use crate::schema::ivr_attrs::dsl::*;
    use crate::schema::ivr_attrs;
    let mut conn = db_connect();

    diesel::insert_into(ivr_attrs::table)
        .values((&ivr_id.eq(a_ivr_id),
                 &name.eq(a_name),
                 &value.eq(a_value)))
        .execute(&mut conn)?;

    Ok(())
}

pub fn add_ivr_entry(domain: i32, a_ivr_id: i32, ds: String, exten: String) -> Result<()> {
    use crate::schema::ivr_entries::dsl::*;
    use crate::schema::ivr_entries;
    let mut conn = db_connect();

    if !ivr_exists(a_ivr_id)? {
        return Err(Error::Fslib("IVR doesn't exist".to_string()));
    }

    let exten = extension::get_extension(&exten, domain)?;

    diesel::insert_into(ivr_entries::table)
        .values((&ivr_id.eq(a_ivr_id),
                 &digits.eq(ds),
                 &dest_exten.eq(exten.exten)))
        .execute(&mut conn)?;

    Ok(())
}

pub fn del_ivr_attr(attr_id: i32) -> Result<()> {
    use crate::schema::ivr_attrs::columns::id;
    use crate::schema::ivr_attrs;
    let mut conn = db_connect();

    diesel::delete(ivr_attrs::table)
        .filter(id.eq(attr_id))
        .execute(&mut conn)?;

    Ok(())
}

pub fn del_ivr_entry(entry_id: i32) -> Result<()> {
    use crate::schema::ivr_entries::columns::id;
    use crate::schema::ivr_entries;
    let mut conn = db_connect();

    diesel::delete(ivr_entries::table)
        .filter(id.eq(entry_id))
        .execute(&mut conn)?;

    Ok(())
}

pub fn attrs(ivr_id: i32) -> Result<Vec<ivr_attrs::IvrAttr>> {
    use crate::schema::ivrs::dsl::*;
    let mut conn = db_connect();

    let ivr = ivrs
        .find(ivr_id)
        .first::<Ivr>(&mut conn)?;

    let attrs = IvrAttr::belonging_to(&ivr)
        .load::<IvrAttr>(&mut conn)?;

    Ok(attrs)
}

pub fn entries(ivr_id: i32) -> Result<Vec<IvrEntry>> {
    use crate::schema::ivrs::dsl::*;
    let mut conn =db_connect();

    let ivr = ivrs
        .find(ivr_id)
        .first::<Ivr>(&mut conn)?;

    let  entries = IvrEntry::belonging_to(&ivr)
        .load::<IvrEntry>(&mut conn)?;

    return Ok(entries);
}
