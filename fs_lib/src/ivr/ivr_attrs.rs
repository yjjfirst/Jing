use diesel::prelude::*;
use crate::error::{Result};
use crate::db_connect;
use crate::schema::{ivr_attrs};
use super::Ivr;

#[derive(Identifiable,Queryable,Associations,Debug)]
#[derive(Clone,PartialEq)]
#[diesel(belongs_to(Ivr))]
pub struct IvrAttr {
    pub id: i32,
    pub ivr_id: i32,
    pub name: String,
    pub value: String
}

#[derive(Insertable)]
#[diesel(table_name=ivr_attrs)]
pub struct NewIvrAttr {
    pub ivr_id: i32,
    pub name: String,
    pub value: String
}

pub fn add_defaults(ivr: i32) -> Result<()> {
    use crate::schema::ivr_attrs::columns::*;
    let mut conn = db_connect();
    diesel::insert_into(ivr_attrs::table)
        .values(&vec![
            (name.eq("greet-long"), value.eq("phrase:demo_ivr_main_menu"), ivr_id.eq(ivr)),
            (name.eq("greet-short"), value.eq("phrase:demo_ivr_main_menu_short"), ivr_id.eq(ivr)),
            (name.eq("invalid-sound"), value.eq("ivr/ivr-that_was_an_invalid_entry.wav"), ivr_id.eq(ivr)),
            (name.eq("exit-sound"), value.eq("voicemail/vm-goodbye.wav"), ivr_id.eq(ivr)),
            (name.eq("confirm-attempts"), value.eq("3"), ivr_id.eq(ivr)),
            (name.eq("timeout"), value.eq("10000"), ivr_id.eq(ivr)),
            (name.eq("inter-digit-timeout"), value.eq("2000"), ivr_id.eq(ivr)),
            (name.eq("max-failures"), value.eq("3"), ivr_id.eq(ivr)),
            (name.eq("max-timeouts"), value.eq("3"), ivr_id.eq(ivr)),
            (name.eq("digit-len"), value.eq("4"), ivr_id.eq(ivr))
        ])
        .execute(&mut conn)?;

    Ok(())
}
