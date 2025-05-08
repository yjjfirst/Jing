use diesel::prelude::*;
use crate::db_connect;
use crate::extension;
use serde::{Serialize, Deserialize};
use crate::schema::{ivr_entries};
use super::{Ivr, ivr_exists};
use crate::error::{Result, Error};

#[derive(Identifiable,Queryable,Associations,Debug, Serialize, Deserialize,AsChangeset)]
#[derive(Clone,PartialEq)]
#[diesel(table_name=ivr_entries)]
#[diesel(belongs_to(Ivr))]
pub struct IvrEntry {
    pub id: i32,
    pub ivr_id: i32,
    pub digits: String,
    pub dest_exten: String
}

pub fn list(ivr_id: i32) -> Result<Vec<IvrEntry>> {
    use crate::schema::ivrs::dsl::*;
    let mut conn = db_connect();

    let ivr = ivrs
        .find(ivr_id)
        .first::<Ivr>(&mut conn)?;

    let  entries = IvrEntry::belonging_to(&ivr)
        .load::<IvrEntry>(&mut conn)?;

    return Ok(entries);
}

pub fn add_entry(domain: i32, a_ivr_id: i32, ds: String, exten: String) -> Result<()> {
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

pub fn update(e: &IvrEntry) -> Result<()> {
    let mut conn = db_connect();
    use crate::schema::ivr_entries;
    use crate::schema::ivr_entries::dsl::*;

    diesel::update(ivr_entries::table)
        .filter(id.eq(e.id))
        .set(e)
        .execute(&mut conn)?;

    Ok(())
}

pub fn del_entries_of(a_ivr_id: i32) -> Result<()> {
    use crate::schema::ivr_entries::columns::ivr_id;
    use crate::schema::ivr_entries;
    let mut conn = db_connect();

    diesel::delete(ivr_entries::table)
        .filter(ivr_id.eq(a_ivr_id))
        .execute(&mut conn)?;

    Ok(())
}

pub fn del_entry(entry_id: i32) -> Result<()> {
    use crate::schema::ivr_entries::columns::id;
    use crate::schema::ivr_entries;
    let mut conn = db_connect();

    diesel::delete(ivr_entries::table)
        .filter(id.eq(entry_id))
        .execute(&mut conn)?;

    Ok(())
}
