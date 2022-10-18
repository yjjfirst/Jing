use diesel::prelude::*;
use crate::error::{Result};
use crate::db_connect;
use crate::schema::{conference_profiles, conference_profile_params};

#[derive(Identifiable,Queryable,Debug,PartialEq)]
#[derive(Clone)]
pub struct ConferenceProfile {
    pub id: i32,
    pub name: String,
    pub description: Option<String>
}

#[derive(Identifiable,Queryable,Associations,Debug)]
#[derive(Clone,PartialEq)]
#[diesel(belongs_to(ConferenceProfile))]
pub struct ConferenceProfileParam {
    pub id: i32,
    pub conference_profile_id: i32,
    pub name: String,
    pub value: String
}

pub fn profiles() -> Result<Vec<ConferenceProfile>> {
    use crate::schema::conference_profiles::dsl::*;
    let mut conn = db_connect();
    let result = conference_profiles
        .load::<ConferenceProfile>(&mut conn)?;

    Ok(result)
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
