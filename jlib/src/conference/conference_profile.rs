use diesel::prelude::*;
use crate::error::{Result};
use crate::db_connect;
use crate::util_macro::{Fields};
use crate::printable::{Printable};
use crate::schema::{conference_profiles};
use super::conference_profile_param::ConferenceProfileParam;
use super::conference_profile_param;

#[derive(Identifiable,Queryable,Debug,PartialEq)]
#[derive(Clone)]
#[derive(Fields)]
pub struct ConferenceProfile {
    pub id: i32,
    pub name: String,
    pub description: String
}


pub fn profiles() -> Result<Vec<ConferenceProfile>> {
    use crate::schema::conference_profiles::dsl::*;
    let mut conn = db_connect();
    let result = conference_profiles
        .load::<ConferenceProfile>(&mut conn)?;

    Ok(result)
}

pub fn add(a_name: &str, a_desc: &str) -> Result<()> {
    use crate::schema::conference_profiles::dsl::*;
    let mut conn = db_connect();
    let inserted: Vec<ConferenceProfile> = diesel::insert_into(conference_profiles)
        .values((name.eq(a_name), description.eq(a_desc)))
        .load(&mut conn)?;

    conference_profile_param::add_defaults(inserted.first().unwrap().id)?;

    Ok(())
}

pub fn del(a_id: i32) -> Result<()> {
    use crate::schema::conference_profiles::columns::id;
    let mut conn = db_connect();

    diesel::delete(conference_profiles::table)
        .filter(id.eq(a_id))
        .execute(&mut conn)?;

    Ok(())
}

pub fn update(a_id: i32, a_name: String, a_desc: String) -> Result<()> {
    use crate::schema::conference_profiles::dsl::*;
    let mut conn = db_connect();

    diesel::update(conference_profiles)
        .filter(id.eq(a_id))
        .set((name.eq(a_name), description.eq(a_desc)))
        .execute(&mut conn)?;

    Ok(())
}

pub fn params(profile_id: i32) -> Result<Vec<ConferenceProfileParam>>{
    use crate::schema::conference_profiles::dsl::*;
    let mut conn = db_connect();
    let profile: ConferenceProfile = conference_profiles
        .find(profile_id)
        .first(&mut conn)?;
    let params = ConferenceProfileParam::belonging_to(&profile)
        .load::<ConferenceProfileParam>(&mut conn)?;

    Ok(params)
}

pub fn get(profile_id: i32) -> Result<ConferenceProfile> {
    use crate::schema::conference_profiles::dsl::*;
    let mut conn = db_connect();

    let result = conference_profiles
        .find(profile_id)
        .first(&mut conn)?;

    Ok(result)
}
