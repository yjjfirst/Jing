pub mod models;

use models::*;
use diesel::prelude::*;
use crate::db_connect;
use crate::schema::sound;
use crate::error::{Result};
use super::extension::{add_extension, del_extension};
use super::domain::*;

pub fn add(domain_id: i32, sound_file_id: i32, name: String, exten: String) -> Result<()> {
    let mut conn = db_connect();
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
        .execute(&mut conn)?;

    Ok(())
}

pub fn del(a_id: i32) -> Result<()> {
    use crate::schema::sound::columns::id;
    let mut conn = db_connect();

    let Sound { exten, .. } = get(a_id)?;
    del_extension(&exten)?;

    diesel::delete(sound::table)
        .filter(id.eq(a_id))
        .execute(&mut conn)?;

    Ok(())
}

pub fn get(a_id: i32) -> Result<Sound> {
    use crate::schema::sound::dsl::*;
    let mut conn = db_connect();

    let result = sound
        .find(a_id)
        .first(&mut conn)?;

    Ok(result)
}

pub fn all() -> Result<Vec<Sound>> {
    use crate::schema::sound::dsl::*;
    let mut conn = db_connect();
    let result = sound
        .load::<Sound>(&mut conn)?;

    Ok(result)
}
