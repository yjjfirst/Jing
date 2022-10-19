use diesel::prelude::*;
use crate::error::{Result};
use crate::db_connect;
use crate::schema::{queues};
use super::extension::{add_extension, del_extension};

#[derive(Identifiable,Queryable,Debug,PartialEq)]
#[derive(Clone)]
pub struct Queue {
    pub id: i32,
    pub name: String,
    pub exten: String,
    pub domain_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name=queues)]
pub struct NewQueue {
    pub name: String,
    pub exten: String,
    pub domain_id: i32,
}

pub fn add(domain_id: i32,
           exten: String,
           name: String) -> Result<()>
{
    let mut conn = db_connect();

    add_extension(exten.as_str(), "queue", domain_id)?;

    let new_queue = NewQueue {
        domain_id,
        exten,
        name
    };

    diesel::insert_into(queues::table)
        .values(&new_queue)
        .execute(&mut conn)?;

    Ok(())
}

pub fn del(a_id: i32) -> Result<()> {
    use crate::schema::queues::columns::id;
    let mut conn = db_connect();

    let Queue {exten,..} = get(a_id)?;
    del_extension(&exten)?;
    diesel::delete(queues::table)
        .filter(id.eq(a_id))
        .execute(&mut conn)?;

    Ok(())
}

pub fn all() -> Result<Vec<Queue>> {
    use crate::schema::queues::dsl::*;
    let mut conn = db_connect();

    let result = queues
        .load::<Queue>(&mut conn)?;

    Ok(result)
}

pub fn get(a_id: i32) -> Result<Queue> {
    use crate::schema::queues::dsl::*;
    let mut conn = db_connect();

    let result = queues
        .find(a_id)
        .first(&mut conn)?;

    Ok(result)
}
