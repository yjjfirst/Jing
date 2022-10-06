pub mod models;

use models::*;
use diesel::prelude::*;
use crate::db_connect;
use crate::schema::sound;
use crate::error::{Result};
use super::extension::{add_extension, del_extension};
use super::domain::*;

pub fn add(domain_id: i32, sound_file_id: i32, name: String, exten: String) -> Result<()> {
    let conn = db_connect();
    let domain = get_domain(domain_id).unwrap();
    add_extension(exten.as_str(), "sound", domain.id)?;

    let new_sound = NewSound {
        name,
        exten,
        domain_id,
        sound_file_id,
    };

    diesel::insert_into(sound::table)
        .values(&new_sound)
        .execute(&conn)?;

    Ok(())
}

pub fn del(a_id: i32) -> Result<()> {
    use crate::schema::sound::columns::id;
    let conn = db_connect();

    diesel::delete(sound::table)
        .filter(id.eq(a_id))
        .execute(&conn)?;

    del_extension();
    Ok(())
}

pub fn all() -> Result<Vec<Sound>> {
    use crate::schema::sound::dsl::*;
    let conn = db_connect();
    let result = sound
        .load::<Sound>(&conn)?;

    Ok(result)
}
