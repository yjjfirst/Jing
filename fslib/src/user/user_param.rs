use diesel::prelude::*;
use super::User;
use crate::error::{Result};
use crate::schema::user_params::*;
use crate::schema::user_params;
use crate::schema::user_params::table;
use crate::params::{Param, Fields};
use crate::db_connect;

#[derive(Identifiable,Queryable,Associations,Debug)]
#[derive(Clone,PartialEq)]
#[derive(Param, Fields)]
#[diesel(belongs_to(User))]
pub struct UserParam {
    #[id]
    pub id: i32,
    #[parent_id]
    pub user_id: i32,
    #[name]
    pub name: String,
    #[value]
    pub value: String
}
