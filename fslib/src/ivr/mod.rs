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
    use crate::schema::ivrs;
    let mut conn = db_connect();

    diesel::insert_into(ivrs::table)
        .values(&new_ivr)
        .execute(&mut conn)?;

    Ok(())
}

pub fn ivr_exists(i: i32) -> Result<bool> {
    use crate::schema::ivrs::dsl::*;
    let  mut conn = db_connect();

    let result = select(exists(ivrs.filter(id.eq(i)))).get_result::<bool>(&mut conn)?;

    Ok(result)
}

pub fn del(i: i32) -> Result<()> {
    use crate::schema::ivrs;
    use crate::schema::ivrs::columns::id;

    let mut conn = db_connect();

    diesel::delete(ivrs::table)
        .filter(id.eq(i))
        .execute(&mut conn)?;

    Ok(())
}

pub fn all() -> Result<Vec<Ivr>> {
    use crate::schema::ivrs::dsl::*;

    let mut conn = db_connect();

    let results = ivrs
        .load::<Ivr>(&mut conn)?;

    Ok(results)
}

pub fn add_ivr_option(a_ivr_id: i32, ds: String, exten: String) -> Result<()> {
    use crate::schema::ivr_options::dsl::*;
    use crate::schema::ivr_options;
    let mut conn = db_connect();
    let domain = domain::get_active()?;

    if !ivr_exists(a_ivr_id)? {
        return Err(Error::Fslib("IVR doesn't exist".to_string()));
    }

    let exten = extension::get_extension(&exten, domain.id)?;

    diesel::insert_into(ivr_options::table)
        .values((&ivr_id.eq(a_ivr_id),
                 &digits.eq(ds),
                 &dest_type.eq(exten.exten_type),
                 &dest_exten.eq(exten.exten)))
        .execute(&mut conn)?;

    Ok(())
}
