use diesel::prelude::*;
use crate::schema::{ivr_entries};
use super::Ivr;

#[derive(Identifiable,Queryable,Associations,Debug)]
#[derive(Clone,PartialEq)]
#[diesel(table_name=ivr_entries)]
#[diesel(belongs_to(Ivr))]
pub struct IvrEntry {
    pub id: i32,
    pub ivr_id: i32,
    pub digits: String,
    pub dest_exten: String
}
