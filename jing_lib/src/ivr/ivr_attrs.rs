use diesel::prelude::*;
use crate::error::{Result};
use crate::db_connect;
use crate::schema::{ivr_attrs};
use serde::{Serialize, Deserialize};
use super::Ivr;

#[derive(Identifiable,Queryable,Associations,Debug, Serialize, Deserialize, AsChangeset)]
#[derive(Clone,PartialEq)]
#[diesel(belongs_to(Ivr))]
pub struct IvrAttr {
    pub id: i32,
    pub ivr_id: i32,
    pub name: String,
    pub value: String
}

#[derive(Insertable)]
#[diesel(table_name=ivr_attrs)]
pub struct NewIvrAttr {
    pub ivr_id: i32,
    pub name: String,
    pub value: String
}

pub fn default_attrs<'a>() -> Vec<(&'a str, &'a str)>{
    let attrs = vec![
        ("greet-long", "3"),
        ("greet-short", "3"),
        ("invalid-sound", "1"),
        ("exit-sound", "2"),
        ("confirm-attempts", "3"),
        ("timeout", "10000"),
        ("inter-digit-timeout", "2000"),
        ("max-failures", "3"),
        ("max-timeouts", "3"),
        ("digit-len", "4")
    ];

    attrs
}

pub fn add_defaults(ivr: i32, _greet_long: &str, _greet_short: &str) -> Result<()> {
    use crate::schema::ivr_attrs::columns::*;
    let mut conn = db_connect();
    let attrs = default_attrs();

    diesel::insert_into(ivr_attrs::table)
        .values(
            attrs.iter().map(|a| {
                (name.eq(a.0.to_string()),
                 value.eq(a.1.to_string()),
                 ivr_id.eq(ivr))
            }).collect::<Vec<_>>()
        )
        .execute(&mut conn)?;

    Ok(())
}

pub fn add_attr(a_ivr_id: i32, a_name: String, a_value: String) -> Result<()> {
    use crate::schema::ivr_attrs::dsl::*;
    use crate::schema::ivr_attrs;
    let mut conn = db_connect();

    diesel::insert_into(ivr_attrs::table)
        .values((&ivr_id.eq(a_ivr_id),
                 &name.eq(a_name),
                 &value.eq(a_value)))
        .execute(&mut conn)?;

    Ok(())
}

pub fn update(i: &IvrAttr) -> Result<()> {
    let mut conn = db_connect();
    use crate::schema::ivr_attrs;
    use crate::schema::ivr_attrs::dsl::*;

    diesel::update(ivr_attrs::table)
        .filter(id.eq(i.id))
        .set(i)
        .execute(&mut conn)?;

    Ok(())
}

pub fn del_attrs_of(a_ivr_id: i32) -> Result<()> {
    use crate::schema::ivr_attrs::columns::ivr_id;
    use crate::schema::ivr_attrs;
    let mut conn = db_connect();

    diesel::delete(ivr_attrs::table)
        .filter(ivr_id.eq(a_ivr_id))
        .execute(&mut conn)?;

    Ok(())

}

pub fn del_attr(attr_id: i32) -> Result<()> {
    use crate::schema::ivr_attrs::columns::id;
    use crate::schema::ivr_attrs;
    let mut conn = db_connect();

    diesel::delete(ivr_attrs::table)
        .filter(id.eq(attr_id))
        .execute(&mut conn)?;

    Ok(())
}

pub fn list(ivr_id: i32) -> Result<Vec<IvrAttr>> {
    use crate::schema::ivrs::dsl::*;
    let mut conn = db_connect();

    let ivr = ivrs
        .find(ivr_id)
        .first::<Ivr>(&mut conn)?;

    let attrs = IvrAttr::belonging_to(&ivr)
        .load::<IvrAttr>(&mut conn)?;

    Ok(attrs)
}
