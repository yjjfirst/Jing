use diesel::prelude::*;
use crate::error::{Result};
use crate::db_connect;
use crate::schema::{conference_controls, conference_control_details};

#[derive(Identifiable,Queryable,Debug,PartialEq)]
#[derive(Clone)]
pub struct ConferenceControl {
    pub id: i32,
    pub name: String,
    pub description: Option<String>
}

#[derive(Identifiable,Queryable,Associations,Debug)]
#[derive(Clone,PartialEq)]
#[diesel(belongs_to(ConferenceControl))]
pub struct ConferenceControlDetail {
    pub id: i32,
    pub conference_control_id: i32,
    pub action: String,
    pub digits: String,
}

pub fn groups() -> Result<Vec<ConferenceControl>>{
    use crate::schema::conference_controls::dsl::*;
    let mut conn = db_connect();
    let result = conference_controls
        .load::<ConferenceControl>(&mut conn)?;

    Ok(result)
}

pub fn group_details(group_id:i32) -> Result<Vec<ConferenceControlDetail>> {
    use crate::schema::conference_controls::dsl::*;

    let mut conn = db_connect();

    let control: ConferenceControl = conference_controls
        .find(group_id)
        .first(&mut conn)?;

    let details = ConferenceControlDetail::belonging_to(&control)
        .load::<ConferenceControlDetail>(&mut conn)?;

    Ok(details)
}
