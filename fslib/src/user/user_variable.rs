use diesel::prelude::*;
use super::User;
use crate::error::{Result};
use crate::schema::user_variables::*;
use crate::schema::user_variables;
use crate::schema::user_variables::table;
use crate::params::{Param, Fields};
use crate::db_connect;
use crate::fieldable::{Fieldable};

#[derive(Identifiable,Queryable,Associations,Debug)]
#[derive(Clone,PartialEq)]
#[derive(Param)]
#[derive(Fields)]
#[diesel(belongs_to(User))]
pub struct UserVariable {
    #[id]
    pub id: i32,
    #[parent_id]
    pub user_id: i32,
    #[name]
    pub name: String,
    #[value]
    pub value: String
}

#[test]
fn user_variable_add_test() {
    UserVariable::add(1, "name", "value").unwrap();
}

#[test]
fn user_variable_del_test() {
    UserVariable::del(1).unwrap();
}

#[test]
fn user_variable_update_test() {
    UserVariable::update(7, "new_name", "new_value").unwrap();
}

#[test]
fn user_variable_fields_test() {
    let p : UserVariable = UserVariable {
        id: 1,
        user_id: 2,
        name: "t".to_string(),
        value: "v".to_string()
    };

    println!("{:?}", p.fields());
    println!("{:?}", p.field_values());
}
