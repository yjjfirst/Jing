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

pub enum ByField {
    Id(i32),
    UserId(String)
}

pub fn add_user<'a> (
    domain: i32,
    user_id: &'a str) -> Result<()> {

    use crate::schema::users;
    let mut conn = db_connect();

    let new_user = NewUser {
        domain_id: domain,
        user_id,
    };

    add_extension(user_id, "user", domain)?;

    let inserted: Vec<User> = diesel::insert_into(users::table)
        .values(&new_user)
        .load(&mut conn)?;

    UserParam::add_defaults(inserted[0].id)?;
    UserVariable::add_defaults(inserted[0].id)?;

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

pub fn users_within(domain: i32) -> Result<Vec<User>>{
    use crate::schema::users::dsl::*;

    let mut conn = db_connect();

    let results: Vec<User> = users
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

pub fn get_user(field: ByField) -> Result<User> {
    use crate::schema::users::dsl::*;

    let mut conn = db_connect();
    match field {
        ByField::Id(i) => {
            Ok(users
                .find(i)
                .first::<User>(&mut conn)?)
        },
        ByField::UserId(u) => {
            Ok(users
                .filter(user_id.eq(u))
                .first::<User>(&mut conn)?)
        }
    }

}

pub fn update_user(u: &User) -> Result<()>{
    let mut conn = db_connect();
    use crate::schema::users;
    use crate::schema::users::dsl::*;
    diesel::update(users::table)
        .filter(id.eq(u.id))
        .set(u)
        .execute(&mut conn)?;

    Ok(())
}

pub fn get_user_params(user_id: i32) -> Result<Vec<UserParam>>{
    let mut conn = db_connect();

    let user = get_user(ByField::Id(user_id))?;
    let params =  UserParam::belonging_to(&user)
        .load::<UserParam>(&mut conn)?;

    Ok(params)

}

pub fn get_user_vars(user_id: i32) -> Result<Vec<UserVariable>> {
    let mut conn = db_connect();

    let user = get_user(ByField::Id(user_id))?;
    let vars = UserVariable::belonging_to(&user)
        .load::<UserVariable>(&mut conn)?;

    Ok(vars)

}
