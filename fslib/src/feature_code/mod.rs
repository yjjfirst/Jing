use diesel::prelude::*;
use crate::error::{Result};
use crate::db_connect;
use crate::schema::{feature_codes};
use crate::util_macro::{Fields};
use crate::printable::{Printable};

#[derive(Identifiable,Queryable,Debug,PartialEq)]
#[derive(Clone)]
#[derive(Fields)]
pub struct FeatureCode {
    pub id: i32,
    pub digits: String,
    pub action: String
}

impl FeatureCode {
    pub fn add(d: &str, a: &str) -> Result<()>{
        use crate::schema::feature_codes::dsl::*;
        let mut conn = db_connect();

        diesel::insert_into(feature_codes)
            .values((digits.eq(d), action.eq(a)))
            .execute(&mut conn)?;

        Ok(())
    }

    pub fn del(a_id: i32) -> Result<()>{
        use crate::schema::feature_codes::dsl::*;
        let mut conn = db_connect();

        diesel::delete(feature_codes)
            .filter(id.eq(a_id))
            .execute(&mut conn)?;

        Ok(())

    }
    pub fn update(a_id: i32, a_digits: &str, a_action: &str) -> Result<()> {
        use crate::schema::feature_codes::dsl::*;
        let mut conn = db_connect();

        diesel::update(feature_codes)
            .filter(id.eq(a_id))
            .set((digits.eq(a_digits), action.eq(a_action)))
            .execute(&mut conn)?;
        Ok(())
    }

    pub fn ls() -> Result<Vec<FeatureCode>>{
        use crate::schema::feature_codes::dsl::*;
        let mut conn = db_connect();
        let result = feature_codes
            .load::<FeatureCode>(&mut conn)?;

        Ok(result)
    }

}
