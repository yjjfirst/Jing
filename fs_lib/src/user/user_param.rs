use diesel::prelude::*;
use super::User;
use crate::error::{Result};
use crate::schema::user_params::*;
use crate::schema::user_params;
use crate::schema::user_params::table;
use crate::util_macro::{Param, Fields};
use crate::db_connect;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use crate::printable::{Printable};
use serde::{Serialize,Deserialize};

#[derive(Identifiable,Queryable,Associations,Debug,Serialize,Deserialize)]
#[derive(Clone,PartialEq)]
#[derive(Param)]
#[derive(Fields)]
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

impl UserParam {
    pub fn add_defaults(uid: i32) -> Result<()>{
        UserParam::add(uid, "password", &UserParam::rand_passwd())?;
        UserParam::add(uid, "vm-password", "0000")?;

        Ok(())
    }

    fn rand_passwd() -> String {
        let password: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect();

        password
    }
}
