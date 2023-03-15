use diesel::prelude::*;
use crate::db_connect;
use crate::error::Result;
use super::conference_profile::ConferenceProfile;
use crate::schema::conference_profile_params;
use crate::schema::conference_profile_params::*;
use crate::schema::conference_profile_params::table;
use crate::util_macro::{Param, Fields};
use crate::printable::{Printable};

#[derive(Identifiable,Queryable,Associations,Debug)]
#[derive(Clone,PartialEq)]
#[derive(Param)]
#[derive(Fields)]
#[diesel(belongs_to(ConferenceProfile))]
pub struct ConferenceProfileParam {
    #[id]
    pub id: i32,
    #[parent_id]
    pub conference_profile_id: i32,
    #[name]
    pub name: String,
    #[value]
    pub value: String
}

pub fn add_defaults(profile_id: i32) -> Result<()>{
    ConferenceProfileParam::add(profile_id, "domain", "$${domain}")?;
    ConferenceProfileParam::add(profile_id, "rate", "8000")?;
    ConferenceProfileParam::add(profile_id, "interval", "20")?;
    ConferenceProfileParam::add(profile_id, "energy-level", "100")?;
    ConferenceProfileParam::add(profile_id, "muted-sound", "conference/conf-muted.wav")?;
    ConferenceProfileParam::add(profile_id, "unmuted-sound", "conference/conf-unmuted.wav")?;
    ConferenceProfileParam::add(profile_id, "alone-sound", "conference/conf-alone.wav")?;
    ConferenceProfileParam::add(profile_id, "moh-sound", "$${hold_music}")?;
    ConferenceProfileParam::add(profile_id, "enter-sound", "tone_stream://%(200,0,500,600,700)")?;
    ConferenceProfileParam::add(profile_id, "exit-sound", "tone_stream://%(500,0,300,200,100,50,25)")?;
    ConferenceProfileParam::add(profile_id, "kicked-sound", "conference/conf-kicked.wav")?;
    ConferenceProfileParam::add(profile_id, "locked-sound", "conference/conf-locked.wav")?;
    ConferenceProfileParam::add(profile_id, "is-locked-sound", "conference/conf-is-locked.wav")?;
    ConferenceProfileParam::add(profile_id, "is-unlocked-sound", "conference/conf-is-unlocked.wav")?;
    ConferenceProfileParam::add(profile_id, "pin-sound", "conference/conf-pin.wav")?;
    ConferenceProfileParam::add(profile_id, "bad-pin-sound", "conference/conf-bad-pin.wav")?;
    ConferenceProfileParam::add(profile_id, "caller-id-name", "$${outbound_caller_name}")?;
    ConferenceProfileParam::add(profile_id, "caller-id-number", "$${outbound_caller_id}")?;
    ConferenceProfileParam::add(profile_id, "comfort-noise", "true")?;
    ConferenceProfileParam::add(profile_id, "caller-controls", "default")?;

    Ok(())
}
