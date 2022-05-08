pub mod models;

use models::*;
use super::user::{get_user_domain, get_user_id};
use super::domain::{get_domain_name};
use diesel::prelude::*;
use crate::db_connect;
use crate::error::{Result, HornetError};

pub fn add_ringgroup(name: String, group_id: String, domain_id: i32, ring_time: Option<i32>, strategy: Option<String>) -> Result<()>{
    use crate::schema::ringing_group;
    let conn = db_connect();
    let new_group = NewRinggroup {
        name: &name,
        group_id: &group_id,
        domain_id,
        ring_time,
        ring_strategy: strategy.as_deref()
    };

    diesel::insert_into(ringing_group::table)
        .values(&new_group)
        .execute(&conn)?;

    Ok(())
}

pub fn del_ringgroup(i: i32) -> Result<()>{
    use crate::schema::ringing_group;
    use crate::schema::ringing_group::columns::id;

    let conn = db_connect();

    diesel::delete(ringing_group::table)
        .filter(id.eq(i))
        .execute(&conn)?;

    Ok(())
}

pub fn all_ringgroup() -> Result<Vec<Ringgroup>>{
    use crate::schema::ringing_group::dsl::*;

    let conn = db_connect();

    let results = ringing_group
        .load::<Ringgroup>(&conn)?;

    Ok(results)
}

pub fn add_ringgroup_member(group: i32, user: i32) -> Result<()> {
    use crate::schema::ringing_group_member::dsl::*;
    
    let conn = db_connect();
    let user_domain = get_user_domain(user)?;
    let ringgroup_domain = get_ringgroup_domain(group)?;

    if let Ok(1) = member_exists(group, user) {
        return Err(HornetError::LogicError("User exist in ringing group".to_string()));
    }
    
    if user_domain == ringgroup_domain {
        diesel::insert_into(ringing_group_member)
            .values((ringing_group_id.eq(group), user_id.eq(user)))
            .execute(&conn)?;
    } else {
        return Err(HornetError::LogicError("User doamin and ringing group domain don't match.".to_string()));
    }

    Ok(())
}

pub fn del_ringgroup_member(group: i32, user: i32) -> Result<()> {
    use crate::schema::ringing_group_member;
    use crate::schema::ringing_group_member::dsl::*;

    let conn = db_connect();

    diesel::delete(ringing_group_member::table)
        .filter(ringing_group_id.eq(group))
        .filter(user_id.eq(user))
        .execute(&conn)?;

    Ok(())
}

pub fn all_ringgroup_member(group: i32) -> Result<Vec<(i32,String,String)>> {
    use crate::schema::ringing_group_member::dsl::*;

    let conn = db_connect();

    let query_results = ringing_group_member
        .filter(ringing_group_id.eq(group))
        .load::<(i32, i32, i32)>(&conn)
        .unwrap();
    let results: Vec<(i32,String,String)> = query_results
        .into_iter()
        .map(|x| {
            let u = get_user_id(x.2).unwrap();
            let dn = get_domain_name(get_ringgroup_domain(x.1).unwrap()).unwrap();
            
            (x.0, u, dn)
        })
        .collect();
             
    Ok(results)
}

fn get_ringgroup_domain(target_ringgroup_id: i32) -> Result<i32> {
    use crate::schema::ringing_group::dsl::*;

    let conn = db_connect();

    let domain = ringing_group
        .select(domain_id)
        .filter(id.eq(target_ringgroup_id))
        .load::<i32>(&conn)?;
    
    Ok(domain[0])
}

fn member_exists(group: i32, user: i32) -> Result<usize> {
    use crate::schema::ringing_group_member::dsl::*;
    let conn = db_connect();

    let result = ringing_group_member
        .filter(ringing_group_id.eq(group))
        .filter(user_id.eq(user))
        .select(user_id)
        .execute(&conn)?;

    Ok(result)
}
