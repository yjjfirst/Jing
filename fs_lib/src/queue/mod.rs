pub mod queue_param;
pub mod agent;
pub mod agent_param;
pub mod tier;

use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use crate::error::{Result};
use crate::db_connect;
use crate::schema::{queues};
use super::extension::{add_extension, del_extension};
use queue_param::{QueueParam};

#[derive(Identifiable,Queryable,Debug,PartialEq,Serialize, Deserialize, AsChangeset)]
#[derive(Clone)]
pub struct Queue {
    pub id: i32,
    pub name: String,
    pub exten: String,
    pub domain_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name=queues)]
pub struct NewQueue<'a> {
    pub name: &'a str,
    pub exten: &'a str,
    pub domain_id: i32,
}

pub fn add(domain_id: i32,
           exten: String,
           name: String) -> Result<i32>
{
    let mut conn = db_connect();

    add_extension(exten.as_str(), "queue", domain_id)?;

    let new_queue = NewQueue {
        domain_id,
        exten: &exten,
        name: &name
    };

    let queue = diesel::insert_into(queues::table)
        .values(&new_queue)
        .get_result::<Queue>(&mut conn)?;

    queue_param::add_defaults(queue.id)?;

    Ok(queue.id)
}

pub fn update(queue: Queue) -> Result<()> {
    use crate::schema::queues;
    use crate::schema::queues::dsl::*;

    let mut conn = db_connect();
    diesel::update(queues::table)
        .filter(id.eq(queue.id))
        .set(queue)
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

pub fn list() -> Result<Vec<Queue>> {
    use crate::schema::queues::dsl::*;
    let mut conn = db_connect();

    let result = queues
        .load::<Queue>(&mut conn)?;

    Ok(result)
}

pub fn queues_in(domain: i32) -> Result<Vec<Queue>> {
    use crate::schema::queues::dsl::*;
    let mut conn = db_connect();

    let result = queues
        .filter(domain_id.eq(domain))
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

pub fn get_by(domain: i32, ext: &str) -> Result<Queue> {
    use crate::schema::queues::dsl::*;
    let mut conn = db_connect();

    let result = queues
        .filter(domain_id.eq(domain))
        .filter(exten.eq(ext))
        .first(&mut conn)?;

    Ok(result)
}

pub fn params(d_id: i32) -> Result<Vec<QueueParam>> {
    use crate::schema::queues::dsl::*;
    let mut conn = db_connect();
    let queue = queues
        .find(d_id)
        .first::<Queue>(&mut conn)?;

    let params = QueueParam::belonging_to(&queue)
        .load::<QueueParam>(&mut conn)?;

    Ok(params)
}
