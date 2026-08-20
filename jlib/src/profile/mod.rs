pub mod models;

use models::*;
use diesel::prelude::*;
use crate::db_connect;
use crate::error::{Result};
use super::gateway::models::Gateway;

pub fn list() -> Result<Vec<Profile>> {
    use crate::schema::profiles::dsl::*;
    let mut conn = db_connect();

    let profs = profiles
        .load::<Profile>(&mut conn)?;

    Ok(profs)
}

pub fn get_profile(profile_id: i32) -> Result<Profile> {
    use crate::schema::profiles::dsl::*;
    let mut conn = db_connect();

    let profile = profiles
        .find(profile_id)
        .get_result::<Profile>(&mut conn)?;

    Ok(profile)
}

pub fn get_profile_by_name(profile_name: &str) -> Result<Profile> {
    use crate::schema::profiles::dsl::*;

    let mut conn = db_connect();
    let profile = profiles
        .filter(name.eq(profile_name))
        .first::<Profile>(&mut conn)?;

    Ok(profile)
}

pub fn get_profile_params(prof_id: i32) -> Result<Vec<ProfileParam>> {
    use crate::schema::profile_params::dsl::*;

    let mut conn = db_connect();
    let profile = get_profile(prof_id)?;

    let results = profile_params
        .filter(profile_id.eq(profile.id))
        .load::<ProfileParam>(&mut conn)?;

    Ok(results)
}

pub fn set_profile_param(param_id: i32, _: &str, param_value: &str) -> Result<()> {
    use crate::schema::profile_params;
    use crate::schema::profile_params::dsl::*;
    let mut conn = db_connect();

    diesel::update(profile_params::table)
        .filter(id.eq(param_id))
        .set(value.eq(param_value))
        .execute(&mut conn)?;

    Ok(())
}

pub fn gateways(profile_id: i32) -> Result<Vec<Gateway>> {
    use crate::schema::profiles::dsl::*;

    let mut conn = db_connect();
    let prof = profiles.find(profile_id).get_result::<Profile>(&mut conn)?;
    let gateways = Gateway::belonging_to(&prof).load(&mut conn)?;

    Ok(gateways)

}
