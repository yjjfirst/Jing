pub mod models;

use models::*;
use diesel::prelude::*;
use crate::db_connect;
use crate::schema::sounds;
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

    diesel::insert_into(sounds::table)
        .values(&new_sound)
        .execute(&mut conn)?;

    Ok(())
}

pub fn del(a_id: i32) -> Result<()> {
    use crate::schema::sounds::columns::id;
    let mut conn = db_connect();

    let Sound { exten, .. } = get(a_id)?;
    del_extension(&exten)?;

    diesel::delete(sounds::table)
        .filter(id.eq(a_id))
        .execute(&mut conn)?;

    Ok(())
}

pub fn get(a_id: i32) -> Result<Sound> {
    use crate::schema::sounds::dsl::*;
    let mut conn = db_connect();

    let result = sounds
        .find(a_id)
        .first(&mut conn)?;

    Ok(result)
}

pub fn get_by(domain: i32, ext: &str) -> Result<Sound> {
    use crate::schema::sounds::dsl::*;
    let mut conn = db_connect();

    let result = sounds
        .filter(domain_id.eq(domain))
        .filter(exten.eq(ext))
        .first(&mut conn)?;

    Ok(result)
}

pub fn all() -> Result<Vec<Sound>> {
    use crate::schema::sounds::dsl::*;
    let mut conn = db_connect();
    let result = sounds
        .load::<Sound>(&mut conn)?;

    Ok(result)
}
