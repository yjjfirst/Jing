use diesel::prelude::*;
use crate::error::{Result};
use crate::db_connect;
use crate::schema::{queue_params};
use super::{Queue};

#[derive(Identifiable,Queryable,Associations,Debug)]
#[derive(Clone,PartialEq)]
#[diesel(belongs_to(Queue))]
pub struct QueueParam {
    pub id: i32,
    pub queue_id: i32,
    pub name: String,
    pub value: String
}

#[derive(Insertable)]
#[diesel(table_name=queue_params)]
pub struct NewQueueParam {
    pub queue_id: i32,
    pub name: String,
    pub value: String
}

pub fn add_defaults(q_id: i32) -> Result<()>{

    let mut conn = db_connect();
    let params = vec![
        ("strategy", "longest-idle-agent"),
        ("moh-sound","$${hold_music}"),
        ("time-base-score","system"),
        ("max-wait-time","0"),
        ("max-wait-time-with-no-agent","0"),
        ("max-wait-time-with-no-agent-time-reached","5"),
        ("tier-rules-apply","false"),
        ("tier-rule-wait-second", "300"),
        ("tier-rule-wait-multiply-level", "true"),
        ("tier-rule-no-agent-no-wait", "false"),
        ("discard-abandoned-after", "60"),
        ("abandoned-resume-allowed", "false")
    ];

    for p in params {
        let new_param = NewQueueParam {
            queue_id: q_id,
            name: p.0.to_string(),
            value: p.1.to_string()
        };

        diesel::insert_into(queue_params::table)
            .values(&new_param)
            .execute(&mut conn)?;
    }

    Ok(())
}
