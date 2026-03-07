use serde::{Serialize};
use diesel::prelude::*;
use chrono::{DateTime, Local, Duration};
use crate::schema::portal_tokens;
use crate::{db_connect, generate_token};
use crate::error::{Result};

#[derive(Queryable, Debug, Serialize)]
#[derive(Clone)]
pub struct PortalToken {
    pub id: i32,
    pub portal_user_id: i32,
    pub token: String,
    pub expire_at: DateTime<Local>,
}

#[derive(Insertable)]
#[diesel(table_name=portal_tokens)]
pub struct NewPortalToken {
    pub portal_user_id: i32,
    pub token: String,
    pub expire_at: DateTime<Local>,
}

pub fn new(user_id: i32) -> Result<String>{
    let token = generate_token(48, Some((true, true, true))).unwrap();
    let new_token = NewPortalToken {
        portal_user_id: user_id,
        token: token.clone(),
        expire_at: Local::now() + Duration::hours(2)
    };

    let mut conn = db_connect();
    diesel::insert_into(portal_tokens::table)
        .values(&new_token)
        .execute(&mut conn)?;

    Ok(token)
}

pub fn is_expired(t: &str) -> bool {
    use crate::schema::portal_tokens::dsl::*;
    let mut conn = db_connect();

    let tok = portal_tokens.filter(token.eq(t))
        .first::<PortalToken>(&mut conn)
        .unwrap();

    if Local::now() <= tok.expire_at {
        false
    } else {
        true
    }
}
