pub mod models;

use models::*;
use diesel::prelude::*;
use crate::db_connect;
use crate::schema::announcements;
use crate::error::{Result};
use super::extension::{add_extension, del_extension};
use super::domain::*;

pub fn add(domain_id: i32, sound_file_id: i32, name: String, exten: String) -> Result<()> {
    let mut conn = db_connect();
    let domain = get_domain(domain_id).unwrap();

    add_extension(exten.as_str(), "sound", domain.id)?;

    let new_sound = NewAnnouncement {
        name,
        exten,
        domain_id,
        sound_file_id,
    };

    diesel::insert_into(announcements::table)
        .values(&new_sound)
        .execute(&mut conn)?;

    Ok(())
}

pub fn del(a_id: i32) -> Result<()> {
    use crate::schema::announcements::columns::id;
    let mut conn = db_connect();

    let Announcement { exten, .. } = get(a_id)?;
    del_extension(&exten)?;

    diesel::delete(announcements::table)
        .filter(id.eq(a_id))
        .execute(&mut conn)?;

    Ok(())
}

pub fn get(a_id: i32) -> Result<Announcement> {
    use crate::schema::announcements::dsl::*;
    let mut conn = db_connect();

    let result = announcements
        .find(a_id)
        .first(&mut conn)?;

    Ok(result)
}

pub fn get_by(domain: i32, ext: &str) -> Result<Announcement> {
    use crate::schema::announcements::dsl::*;
    let mut conn = db_connect();

    let result = announcements
        .filter(domain_id.eq(domain))
        .filter(exten.eq(ext))
        .first(&mut conn)?;

    Ok(result)
}

pub fn list() -> Result<Vec<Announcement>> {
    use crate::schema::announcements::dsl::*;
    let mut conn = db_connect();
    let result = announcements
        .order(id.asc())
        .load::<Announcement>(&mut conn)?;

    Ok(result)
}

pub fn update(s: &Announcement) -> Result<()> {
    let mut conn = db_connect();
    use crate::schema::announcements;
    use crate::schema::announcements::dsl::*;

    diesel::update(announcements::table)
        .filter(id.eq(s.id))
        .set(s)
        .execute(&mut conn)?;

    Ok(())
}
