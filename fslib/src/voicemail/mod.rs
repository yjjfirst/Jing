pub mod models;

use models::*;
use diesel::prelude::*;
use crate::db_connect;
use crate::schema::voicemails;
use crate::error::{Result};

pub fn add_voicemail(user_id: i32, password: String, email: Option<String>) -> Result<()> {
    let mut conn = db_connect();
    let new_voicemail = NewVoicemail {
        user_id,
        password: &password,
        email: email.as_deref(),
    };

    diesel::insert_into(voicemails::table)
        .values(&new_voicemail)
        .execute(&mut conn)?;

    Ok(())
}

pub fn get_voicemail(a_id: i32) -> Result<Voicemail> {
    use crate::schema::voicemails::dsl::*;

    let mut conn = db_connect();

    let results = voicemails
        .filter(user_id.eq(a_id))
        .first::<Voicemail>(&mut conn)?;

    Ok(results)
}

pub fn all_voicemails() -> Result<Vec<Voicemail>> {
    use crate::schema::voicemails::dsl::*;

    let mut conn = db_connect();

    let results = voicemails
        .load::<Voicemail>(&mut conn)?;

    Ok(results)
}

pub fn del_voicemail(vm_id: i32) -> Result<()>{
    use crate::schema::voicemails::columns::*;

    let mut conn = db_connect();

    diesel::delete(voicemails::table)
        .filter(id.eq(vm_id))
        .execute(&mut conn)?;

    Ok(())
}
