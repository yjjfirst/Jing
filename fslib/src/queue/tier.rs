use diesel::prelude::*;
use crate::error::{Result};
use crate::db_connect;
use crate::schema::{tiers};

#[derive(Identifiable,Queryable,Debug,PartialEq)]
#[derive(Clone)]
pub struct Tier {
    pub id: i32,
    pub agent_id: i32,
    pub queue_id: i32,
    pub level: i32,
    pub position: i32
}

#[derive(Insertable)]
#[diesel(table_name=tiers)]
pub struct NewTier {
    pub agent_id: i32,
    pub queue_id: i32,
    pub level: i32,
    pub position: i32,
}

pub fn add(agent_id: i32, queue_id: i32, level: i32, position: i32) -> Result<()>{
    let mut conn = db_connect();

    let tiers = NewTier {
        agent_id,
        queue_id,
        level,
        position
    };

    diesel::insert_into(tiers::table)
        .values(&tiers)
        .execute(&mut conn)?;

    Ok(())
}

pub fn del(tier_id: i32) -> Result<()>{
    use crate::schema::tiers::columns::id;
    let mut conn = db_connect();

    diesel::delete(tiers::table)
        .filter(id.eq(tier_id))
        .execute(&mut conn)?;

    Ok(())
}

pub fn all() -> Result<Vec<Tier>> {
    use crate::schema::tiers::dsl::*;
    let mut conn = db_connect();

    let result = tiers
        .load::<Tier>(&mut conn)?;

    Ok(result)
}
