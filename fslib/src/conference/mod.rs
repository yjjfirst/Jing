pub mod conference_control;
pub mod conference_profile;
pub mod conference_profile_param;
pub mod conference_control_detail;

use diesel::prelude::*;
use crate::error::{Result};
use crate::db_connect;
use crate::schema::{conferences};
use super::extension::{add_extension, del_extension};

#[derive(Identifiable,Queryable,Debug,PartialEq)]
#[derive(Clone)]
pub struct Conference {
    pub id: i32,
    pub exten: String,
    pub name: String,
    pub domain_id: i32,
    pub conference_profile_id: i32,
    pub description: Option<String>
}

#[derive(Insertable)]
#[diesel(table_name=conferences)]
pub struct NewConference {
    pub exten: String,
    pub name: String,
    pub domain_id: i32,
    pub conference_profile_id :i32,
    pub description: Option<String>
}

pub fn add(domain_id: i32,
           conference_profile_id: i32,
           exten: String,
           name: String,
           description: Option<String>) -> Result<()>{
    let mut conn = db_connect();

    add_extension(exten.as_str(), "conference", domain_id)?;

    let new_conference = NewConference {
        domain_id,
        conference_profile_id,
        exten,
        name,
        description
    };

    diesel::insert_into(conferences::table)
        .values(&new_conference)
        .execute(&mut conn)?;

    Ok(())
}

pub fn del(a_id: i32) -> Result<()>{
    use crate::schema::conferences::columns::id;
    let mut conn = db_connect();

    let Conference {exten, ..} = get(a_id)?;
    del_extension(&exten)?;

    diesel::delete(conferences::table)
        .filter(id.eq(a_id))
        .execute(&mut conn)?;

    Ok(())
}

pub fn all() -> Result<Vec<Conference>>{
    use crate::schema::conferences::dsl::*;
    let mut conn = db_connect();

    let result = conferences
        .load::<Conference>(&mut conn)?;

    Ok(result)
}

pub fn get(a_id: i32) -> Result<Conference> {
    use crate::schema::conferences::dsl::*;
    let mut conn = db_connect();

    let result = conferences
        .find(a_id)
        .first(&mut conn)?;

    Ok(result)
}

pub fn get_by(domain: i32, ext: &str) -> Result<Conference> {
    use crate::schema::conferences::dsl::*;
    let mut conn = db_connect();

    let result = conferences
        .filter(domain_id.eq(domain))
        .filter(exten.eq(ext))
        .first(&mut conn)?;

    Ok(result)
}
