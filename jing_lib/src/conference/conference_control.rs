use diesel::prelude::*;
use crate::error::{Result};
use crate::db_connect;
use crate::schema::conference_controls;
use crate::util_macro::{Fields};
use crate::printable::{Printable};
use super::conference_control_detail::ConferenceControlDetail;
use super::conference_control_detail;

#[derive(Identifiable,Queryable,Debug,PartialEq)]
#[derive(Clone)]
#[derive(Fields)]
pub struct ConferenceControl {
    pub id: i32,
    pub name: String,
    pub description: String
}

pub fn groups() -> Result<Vec<ConferenceControl>>{
    use crate::schema::conference_controls::dsl::*;
    let mut conn = db_connect();
    let result = conference_controls
        .load::<ConferenceControl>(&mut conn)?;

    Ok(result)
}

pub fn details(group_id:i32) -> Result<Vec<ConferenceControlDetail>> {
    use crate::schema::conference_controls::dsl::*;

    let mut conn = db_connect();

    let control: ConferenceControl = conference_controls
        .find(group_id)
        .first(&mut conn)?;

    let details = ConferenceControlDetail::belonging_to(&control)
        .load::<ConferenceControlDetail>(&mut conn)?;

    Ok(details)
}

pub fn add(a_name: &str, a_desc: &str) -> Result<()> {
    use crate::schema::conference_controls::dsl::*;
    let mut conn = db_connect();
    let inserted : Vec<ConferenceControl> = diesel::insert_into(conference_controls)
        .values((name.eq(a_name), description.eq(a_desc)))

        .load(&mut conn)?;

    conference_control_detail::add_defaults(inserted.first().unwrap().id)?;

    Ok(())
}

pub fn del(control_id: i32) -> Result<()> {
    use crate::schema::conference_controls::columns::id;
    let mut conn = db_connect();

    diesel::delete(conference_controls::table)
        .filter(id.eq(control_id))
        .execute(&mut conn)?;

    Ok(())
}

pub fn update(a_id: i32, a_name: &str, a_desc: &str) -> Result<()> {
    use crate::schema::conference_controls::dsl::*;
    let mut conn= db_connect();

    diesel::update(conference_controls)
        .filter(id.eq(a_id))
        .set((name.eq(a_name), description.eq(a_desc)))
        .execute(&mut conn)?;

    Ok(())
}
