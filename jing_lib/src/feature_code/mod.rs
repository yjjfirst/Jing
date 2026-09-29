use diesel::prelude::*;
use crate::error::{Result};
use crate::db_connect;
use crate::schema::{feature_codes};
use crate::util_macro::{Fields};
use crate::printable::{Printable};
use crate::extension::{add_extension, del_extension};

#[derive(Identifiable,Queryable,Debug,PartialEq)]
#[derive(Clone)]
#[derive(Fields)]
pub struct FeatureCode {
    pub id: i32,
    pub domain_id: i32,
    pub digits: String,
    pub action: String
}

impl FeatureCode {
    pub fn add(d_id: i32, d: &str, a: &str) -> Result<()>{
        use crate::schema::feature_codes::dsl::*;
        let mut conn = db_connect();

        add_extension(d, "feature_code", d_id)?;

        diesel::insert_into(feature_codes)
            .values((domain_id.eq(d_id),digits.eq(d), action.eq(a)))
            .execute(&mut conn)?;

        Ok(())
    }

    pub fn del(a_id: i32) -> Result<()>{
        use crate::schema::feature_codes::dsl::*;
        let mut conn = db_connect();
        let code = FeatureCode::get(a_id)?;

        diesel::delete(feature_codes)
            .filter(id.eq(a_id))
            .execute(&mut conn)?;

        del_extension(&code.digits)?;

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

    pub fn get(feature_code_id: i32) -> Result<FeatureCode> {
        use crate::schema::feature_codes::dsl::*;
        let mut conn = db_connect();

        let code = feature_codes
            .find(feature_code_id)
            .first::<FeatureCode>(&mut conn)?;

        Ok(code)
    }

    pub fn get_by(a_digits: String) -> Result<FeatureCode> {
        use crate::schema::feature_codes::dsl::*;
        let mut conn = db_connect();

        let code = feature_codes
            .filter(digits.eq(a_digits))
            .first::<FeatureCode>(&mut conn)?;

        Ok(code)
    }
}
