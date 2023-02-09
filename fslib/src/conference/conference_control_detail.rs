use diesel::prelude::*;
use crate::error::{Result};
use crate::db_connect;
use crate::schema::conference_control_details;
use crate::schema::conference_control_details::*;
use crate::schema::conference_control_details::table;

use super::conference_control::ConferenceControl;
use crate::util_macro::{Param, Fields};
use crate::printable::{Printable};

#[derive(Identifiable,Queryable,Associations,Debug)]
#[derive(Clone,PartialEq)]
#[derive(Param)]
#[derive(Fields)]
#[diesel(belongs_to(ConferenceControl))]
pub struct ConferenceControlDetail {
    #[id]
    pub id: i32,
    #[parent_id]
    pub conference_control_id: i32,
    #[name]
    pub action: String,
    #[value]
    pub digits: String,
}

pub fn add_defaults(control_id: i32) -> Result<()> {
    ConferenceControlDetail::add(control_id, "mute", "0")?;
    ConferenceControlDetail::add(control_id, "deaf mute", "*")?;
    ConferenceControlDetail::add(control_id, "energy up", "9")?;
    ConferenceControlDetail::add(control_id, "energy equ", "8")?;
    ConferenceControlDetail::add(control_id, "energy dn", "7")?;
    ConferenceControlDetail::add(control_id, "vol talk up", "3")?;
    ConferenceControlDetail::add(control_id, "vol talk zero", "2")?;
    ConferenceControlDetail::add(control_id, "vol talk dn", "1")?;
    ConferenceControlDetail::add(control_id, "vol listen up", "6")?;
    ConferenceControlDetail::add(control_id, "vol listen zero", "5")?;
    ConferenceControlDetail::add(control_id, "vol listen dn", "4")?;
    ConferenceControlDetail::add(control_id, "hangup", "#")?;

    Ok(())
}
