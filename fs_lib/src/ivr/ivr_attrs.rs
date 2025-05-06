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

pub fn add_defaults(ivr: i32, greet_long: &str, greet_short: &str) -> Result<()> {
    use crate::schema::ivr_attrs::columns::*;
    let mut conn = db_connect();
    diesel::insert_into(ivr_attrs::table)
        .values(&vec![
            (name.eq("greet-long"), value.eq(""), ivr_id.eq(ivr)),
            (name.eq("greet-short"), value.eq(""), ivr_id.eq(ivr)),
            (name.eq("invalid-sound"), value.eq("1"), ivr_id.eq(ivr)),
            (name.eq("exit-sound"), value.eq("2"), ivr_id.eq(ivr)),
            (name.eq("confirm-attempts"), value.eq("3"), ivr_id.eq(ivr)),
            (name.eq("timeout"), value.eq("10000"), ivr_id.eq(ivr)),
            (name.eq("inter-digit-timeout"), value.eq("2000"), ivr_id.eq(ivr)),
            (name.eq("max-failures"), value.eq("3"), ivr_id.eq(ivr)),
            (name.eq("max-timeouts"), value.eq("3"), ivr_id.eq(ivr)),
            (name.eq("digit-len"), value.eq("4"), ivr_id.eq(ivr))
        ])
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
