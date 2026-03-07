use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::schema::portal_users;
use crate::db_connect;
use crate::error::{Result};
use super::portal_token;

#[derive(Identifiable, AsChangeset, Queryable, Debug, PartialEq, Serialize, Deserialize)]
#[derive(Clone)]
pub struct PortalUser {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name=portal_users)]
pub struct NewPortalUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

pub fn add(username: &str, password: &str) -> Result<()> {
    let mut conn = db_connect();
    let user = NewPortalUser {
        username, password
    };

    diesel::insert_into(portal_users::table)
        .values(&user)
        .execute(&mut conn)?;

    Ok(())
}

pub fn del(portal_user_id: i32) -> Result<()> {
    use crate::schema::portal_users::columns::id;

    let mut conn = db_connect();
    diesel::delete(portal_users::table)
        .filter(id.eq(portal_user_id))
        .execute(&mut conn)?;

    Ok(())
}

pub fn authorize(name: &str, passwd: &str) -> Result<String> {
    use crate::schema::portal_users::dsl::*;
    let mut conn = db_connect();

    let users: Vec<PortalUser> = portal_users
        .filter(username.eq(name))
        .load(&mut conn)?;

    if users.len() == 0 {
        return Ok("".to_string());
    }

    let user = users.get(0).unwrap();
    if user.password != passwd {
        return Ok("".to_string());
    }

    let token = portal_token::new(user.id).unwrap();

    Ok(token)
}
