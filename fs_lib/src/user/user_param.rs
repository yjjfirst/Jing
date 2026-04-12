use diesel::prelude::*;
use super::User;
use crate::error::{Result};
use crate::schema::user_params::*;
use crate::schema::user_params;
use crate::schema::user_params::table;
use crate::util_macro::{Param, Fields};
use crate::{db_connect, generate_token};
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
        UserParam::add(uid,
                       "password",
                       &generate_token(12, Some((true, true, true))).unwrap())?;
        UserParam::add(uid,
                       "vm-password",
                       &generate_token(4, Some((false, false, true))).unwrap())?;

        Ok(())
    }

    pub fn defaults() -> Vec<(String, String)> {
        vec![
            ("password".to_string(), generate_token(12, Some((true, true, true))).unwrap()),
            ("vm_password".to_string(), generate_token(4, Some((false, false, true))).unwrap())
        ]
    }
}
