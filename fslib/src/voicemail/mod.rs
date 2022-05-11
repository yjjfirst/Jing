pub mod models;

use models::*;
use diesel::prelude::*;
use crate::db_connect;
use crate::schema::voicemail;
use crate::error::{Result};

pub fn add_voicemail(user_id: i32, password: String, email: Option<String>) -> Result<()> {
    let conn = db_connect();
    let new_voicemail = NewVoicemail {
        user_id,
        password: &password,
        email: email.as_deref(),
    };

    diesel::insert_into(voicemail::table)
        .values(&new_voicemail)
        .execute(&conn)?;

    Ok(())
}

pub fn get_voicemail(a_id: i32) -> Result<Voicemail> {
    use crate::schema::voicemail::dsl::*;

    let conn = db_connect();

    let results = voicemail
        .filter(user_id.eq(a_id))
        .first::<Voicemail>(&conn)?;

    Ok(results)
}

pub fn all_voicemails() -> Result<Vec<Voicemail>> {
    use crate::schema::voicemail::dsl::*;

    let conn = db_connect();

    let results = voicemail
        .load::<Voicemail>(&conn)?;

    Ok(results)
}

pub fn del_voicemail(vm_id: i32) -> Result<()>{
    use crate::schema::voicemail::columns::*;

    let conn = db_connect();

    diesel::delete(voicemail::table)
        .filter(id.eq(vm_id))
        .execute(&conn)?;

    Ok(())
}
