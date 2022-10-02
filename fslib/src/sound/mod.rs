pub mod models;

use models::*;
use diesel::prelude::*;
use crate::db_connect;
use crate::schema::sound;
use crate::domain;
use crate::error::{Result};

pub fn add(name: String, path: String, desc: String) -> Result<()>{
    let domain = domain::get_active().unwrap();
    let conn = db_connect();
    let new_sound = NewSound {
        name: &name,
        path: &path,
        domain_id: domain.id,
        desc: Some(&desc)
    };
    diesel::insert_into(sound::table)
        .values(&new_sound)
        .execute(&conn)?;

    Ok(())
}

pub fn all() -> Result<Vec<Sound>>{
    use crate::schema::sound::dsl::*;

    let conn = db_connect();
    let result = sound
        .load::<Sound>(&conn)?;

    Ok(result)
}

pub fn del(a_id: i32) -> Result<()>{
    use crate::schema::sound::columns::id;

    let conn = db_connect();
    diesel::delete(sound::table)
        .filter(id.eq(a_id))
        .execute(&conn)?;

    Ok(())
}
