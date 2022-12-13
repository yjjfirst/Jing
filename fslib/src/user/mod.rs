pub mod models;
pub mod user_variable;
pub mod user_param;

use models::*;
use diesel::prelude::*;
use crate::db_connect;
use crate::error::{Result};
use crate::extension::{add_extension, del_extension};
use user_param::{UserParam};
use user_variable::{UserVariable};

pub fn add_user<'a> (
    domain: i32,
    user_id: &'a str,
    password: &'a str) -> Result<()> {

    use crate::schema::users;
    let mut conn = db_connect();

    let new_user = NewUser {
        domain_id: domain,
        user_id,
        password,
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

pub fn users_within_domain(domain: i32) -> Result<Vec<(i32, String, String, String)>>{
    use crate::schema::users;
    use crate::schema::domains;
    use crate::schema::users::dsl::*;

    let mut conn = db_connect();

    let results: Vec<(i32, String, String, String)> = users::table.inner_join(domains::table)
        .select((users::id, users::user_id, users::password, domains::domain_name))
        .filter(domain_id.eq(domain))
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

pub fn get_user(db_id: i32) -> Result<User>{
    use crate::schema::users::dsl::*;

    let mut conn = db_connect();
    let user = users
        .find(db_id)
        .first::<User>(&mut conn)?;

    Ok(user)
}

pub fn get_user_params(user_id: i32) -> Result<Vec<UserParam>>{
    let mut conn = db_connect();

    let user = get_user(user_id)?;
    let params =  UserParam::belonging_to(&user)
        .load::<UserParam>(&mut conn)?;

    Ok(params)

}

pub fn get_user_vars(user_id: i32) -> Result<Vec<UserVariable>> {
    let mut conn = db_connect();

    let user = get_user(user_id)?;
    let vars = UserVariable::belonging_to(&user)
        .load::<UserVariable>(&mut conn)?;

    Ok(vars)

}
