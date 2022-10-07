pub mod models;

use models::*;
use diesel::prelude::*;
use diesel::dsl::*;
use crate::db_connect;
use crate::extension;
use crate::domain;
use crate::error::{Result, Error};

pub enum DestType {
    User(i32),
    Ringgroup(i32)
}

pub fn add(new_ivr: NewIvr) -> Result<()> {
    use crate::schema::ivr;
    let mut conn = db_connect();

    diesel::insert_into(ivr::table)
        .values(&new_ivr)
        .execute(&mut conn)?;

    Ok(())
}

pub fn ivr_exists(i: i32) -> Result<bool> {
    use crate::schema::ivr::dsl::*;
    let  mut conn = db_connect();

    let result = select(exists(ivr.filter(id.eq(i)))).get_result::<bool>(&mut conn)?;

    Ok(result)
}

pub fn del(i: i32) -> Result<()> {
    use crate::schema::ivr;
    use crate::schema::ivr::columns::id;

    let mut conn = db_connect();

    diesel::delete(ivr::table)
        .filter(id.eq(i))
        .execute(&mut conn)?;

    Ok(())
}

pub fn all() -> Result<Vec<Ivr>> {
    use crate::schema::ivr::dsl::*;

    let mut conn = db_connect();

    let results = ivr
        .load::<Ivr>(&mut conn)?;

    Ok(results)
}

pub fn add_ivr_option(a_ivr_id: i32, ds: String, exten: String) -> Result<()> {
    use crate::schema::ivr_option::dsl::*;
    use crate::schema::ivr_option;
    let mut conn = db_connect();
    let domain = domain::get_active()?;

    if !ivr_exists(a_ivr_id)? {
        return Err(Error::Fslib("IVR doesn't exist".to_string()));
    }

    let exten = extension::get_extension(&exten, domain.id)?;

    diesel::insert_into(ivr_option::table)
        .values((&ivr_id.eq(a_ivr_id),
                 &digits.eq(ds),
                 &dest_type.eq(exten.exten_type),
                 &dest_exten.eq(exten.exten)))
        .execute(&mut conn)?;

    Ok(())
}
