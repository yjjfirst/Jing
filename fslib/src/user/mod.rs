pub mod models;

use models::*;
use diesel::prelude::*;
use crate::db_connect;
use crate::error::{Error, Result};
use crate::extension::{add_extension, del_extension};

pub fn add_user<'a> (
    domain_id: i32,
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

    use crate::schema::user;
    let conn = db_connect();
    let new_user = NewUser {
        domain_id,
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

    diesel::insert_into(user::table)
        .values(&new_user)
        .execute(&conn)?;

    add_extension(user_id, "user")?;

    Ok(())
}


pub fn del_user(user: &str) -> Result<()> {
    use crate::schema::user;
    use crate::schema::user::columns::user_id;

    let conn = db_connect();

    diesel::delete(user::table)
        .filter(user_id.eq(user))
        .execute(&conn)?;

    del_extension(user)?;
    Ok(())
}

pub fn all_users_with_domain() -> Result<Vec<(i32, String, String, String)>>{
    use crate::schema::user;
    use crate::schema::domain;

    let conn = db_connect();

    let results: Vec<(i32, String, String, String)> = user::table.inner_join(domain::table)
        .select((user::id, user::user_id, user::password, domain::domain_name))
        .load(&conn)?;

    Ok(results)
}

pub fn all_users() -> Result<Vec<User>> {
    use crate::schema::user::dsl::*;

    let conn = db_connect();

    let results = user
        .load::<User>(&conn)?;


    Ok(results)
}

pub fn get_user_domain(a_user_id: i32) -> Result<i32>{
    use crate::schema::user::dsl::*;

    let conn = db_connect();
    let mut domains = user
        .select(domain_id)
        .filter(id.eq(a_user_id))
        .load::<i32>(&conn)?;

    if let Some(d) = domains.pop() {
        Ok(d)
    } else {
        Err(Error::Fslib("User doesn't exist".to_string()))
    }
}

pub fn get_user_id(db_id: i32) -> Result<String>{
    use crate::schema::user::dsl::*;

    let conn = db_connect();
    let users = user
        .select(user_id)
        .filter(id.eq(db_id))
        .load::<String>(&conn)?;

    Ok(users[0].to_string())
}
