pub mod models;

use models::*;
use super::user::{get_user, ByField};
use super::domain::{get_domain};
use super::extension::{add_extension, del_extension};
use diesel::prelude::*;
use crate::db_connect;
use crate::error::{Result, Error};

pub fn add_ringgroup(domain: i32,
                     name: String,
                     group_id: String,
                     desc: Option<String>,
                     ring_time: Option<i32>,
                     strategy: Option<String>) -> Result<Ringgroup>{
    use crate::schema::ringing_groups;

    let mut conn = db_connect();
    let new_group = NewRinggroup {
        name: &name,
        group_id: &group_id,
        description: desc,
        domain_id: domain,
        ring_time,
        ring_strategy: strategy.as_deref()
    };


    let inserted = diesel::insert_into(ringing_groups::table)
        .values(&new_group)
        .get_result::<Ringgroup>(&mut conn)?;

    add_extension(group_id.as_str(), "ringgroup", domain)?;

    Ok(inserted)
}

pub fn del_ringgroup(i: i32) -> Result<()>{
    use crate::schema::ringing_groups;
    use crate::schema::ringing_groups::columns::id;

    let group = get(i).unwrap();
    let extension = group.group_id.clone();
    let mut conn = db_connect();

    diesel::delete(ringing_groups::table)
        .filter(id.eq(i))
        .execute(&mut conn)?;

    del_extension(extension.as_str())?;

    Ok(())
}

pub fn all_ringgroup() -> Result<Vec<Ringgroup>>{
    use crate::schema::ringing_groups::dsl::*;

    let mut conn = db_connect();

    let results = ringing_groups
        .load::<Ringgroup>(&mut conn)?;

    Ok(results)
}

pub fn groups_in_domain(domain: i32) -> Vec<Ringgroup> {
    use crate::schema::ringing_groups::dsl::*;
    let mut conn = db_connect();

    ringing_groups
        .filter(domain_id.eq(domain))
        .load::<Ringgroup>(&mut conn).unwrap()
}

pub fn get_by(domain: i32, exten: &str) -> Result<Ringgroup> {
    use crate::schema::ringing_groups::dsl::*;
    let mut conn = db_connect();

    let result = ringing_groups
        .filter(domain_id.eq(domain))
        .filter(group_id.eq(exten))
        .first(&mut conn)?;

    Ok(result)
}

pub fn get(target_ringgroup_id: i32) -> Result<Ringgroup> {
    use crate::schema::ringing_groups::dsl::*;

    let mut conn = db_connect();

    let mut groups = ringing_groups
        .filter(id.eq(target_ringgroup_id))
        .load::<Ringgroup>(&mut conn)?;

    if let Some(g) = groups.pop() {
        Ok(g)
    } else {
        Err(Error::Fslib("Ringgroup doesn't exist".to_string()))
    }
}

pub fn update(group: &Ringgroup) -> Result<()> {
    use crate::schema::ringing_groups;
    use crate::schema::ringing_groups::dsl::*;

    let mut conn = db_connect();
    println!("{:?}", group);
    diesel::update(ringing_groups::table)
        .filter(id.eq(group.id))
        .set(group)
        .execute(&mut conn)?;

    Ok(())
}

pub fn add_ringgroup_member(group: i32, uid: i32) -> Result<()> {
    use crate::schema::ringing_group_members::dsl::*;

    let mut conn = db_connect();
    let user  = get_user(ByField::Id(uid))?;
    let ringgroup = get(group)?;
    let ringgroup_domain = ringgroup.domain_id;

    if let Ok(1) = member_exists(group, uid) {
        return Err(Error::Fslib("User exist in ringing group".to_string()));
    }

    if user.domain_id == ringgroup_domain {
        diesel::insert_into(ringing_group_members)
            .values((ringing_group_id.eq(group), user_id.eq(uid)))
            .execute(&mut conn)?;
    } else {
        return Err(Error::Fslib("User doamin and ringing group domain don't match.".to_string()));
    }

    Ok(())
}

pub fn del_ringgroup_member(group: i32, user: i32) -> Result<()> {
    use crate::schema::ringing_group_members;
    use crate::schema::ringing_group_members::dsl::*;

    let mut conn = db_connect();

    diesel::delete(ringing_group_members::table)
        .filter(ringing_group_id.eq(group))
        .filter(user_id.eq(user))
        .execute(&mut conn)?;

    Ok(())
}

pub fn members(group: i32) -> Result<Vec<String>> {
    use crate::schema::ringing_group_members::dsl::*;

    let mut conn = db_connect();

    let query_results = ringing_group_members
        .filter(ringing_group_id.eq(group))
        .load::<(i32, i32, i32)>(&mut conn)
        .unwrap();
    let results: Vec<String> = query_results
        .into_iter()
        .map(|x| {
            let u = get_user(ByField::Id(x.2)).unwrap();
            u.user_id
        })
        .collect();

    Ok(results)
}

fn member_exists(group: i32, user: i32) -> Result<usize> {
    use crate::schema::ringing_group_members::dsl::*;
    let mut conn = db_connect();

    let result = ringing_group_members
        .filter(ringing_group_id.eq(group))
        .filter(user_id.eq(user))
        .select(user_id)
        .execute(&mut conn)?;

    Ok(result)
}
