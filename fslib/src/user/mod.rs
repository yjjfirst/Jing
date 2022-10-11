pub mod models;

use models::*;
use diesel::prelude::*;
use crate::db_connect;
use crate::error::{Error, Result};
use crate::extension::{add_extension, del_extension};

pub fn add_user<'a> (
    domain: i32,
    user_id: &'a str,
    password: &'a str,
    number_alias: Option<&'a str> ,
    mailbox: Option<&'a str>,
    cidr: Option<&'a str>,
    toll_allow: Option<&'a str>,
    user_context: Option<&'a str>,
    default_gateway: Option<&'a str>,
    effective_caller_id_name: Option<&'a str>,
    effective_caller_id_number: Option<&'a str>,
    outbound_caller_id_name : Option<&'a str>,
    outbound_caller_id_number: Option<&'a str>,
    callgroup: Option<&'a str>,
    uservar1: Option<&'a str>,
    uservar2: Option<&'a str>,
    uservar3: Option<&'a str>) -> Result<()> {

    use crate::schema::users;
    let mut conn = db_connect();

    let new_user = NewUser {
        domain_id: domain,
        number_alias,
        mailbox,
        cidr,
        user_id,
        password,
        toll_allow,
        user_context,
        default_gateway,
        effective_caller_id_name,
        effective_caller_id_number,
        outbound_caller_id_name,
        outbound_caller_id_number,
        callgroup,
        uservar1,
        uservar2,
        uservar3,

    };

    add_extension(user_id, "user", domain)?;
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn)?;

    Ok(())
}


pub fn del_user(user: &str) -> Result<()> {
    use crate::schema::users;
    use crate::schema::users::columns::user_id;

    let mut conn = db_connect();

    diesel::delete(users::table)
        .filter(user_id.eq(user))
        .execute(&mut conn)?;

    del_extension(user)?;
    Ok(())
}

pub fn all_users_with_domain() -> Result<Vec<(i32, String, String, String)>>{
    use crate::schema::users;
    use crate::schema::domains;

    let mut conn = db_connect();

    let results: Vec<(i32, String, String, String)> = users::table.inner_join(domains::table)
        .select((users::id, users::user_id, users::password, domains::domain_name))
        .load(&mut conn)?;

    Ok(results)
}

pub fn all_users() -> Result<Vec<User>> {
    use crate::schema::users::dsl::*;

    let mut conn = db_connect();

    let results = users
        .load::<User>(&mut conn)?;


    Ok(results)
}

pub fn get_user_domain(a_user_id: i32) -> Result<i32>{
    use crate::schema::users::dsl::*;

    let mut conn = db_connect();
    let mut domains = users
        .select(domain_id)
        .filter(id.eq(a_user_id))
        .load::<i32>(&mut conn)?;

    if let Some(d) = domains.pop() {
        Ok(d)
    } else {
        Err(Error::Fslib("User doesn't exist".to_string()))
    }
}

pub fn get_user_id(db_id: i32) -> Result<String>{
    use crate::schema::users::dsl::*;

    let mut conn = db_connect();
    let us = users
        .select(user_id)
        .filter(id.eq(db_id))
        .load::<String>(&mut conn)?;

    Ok(us[0].to_string())
}
