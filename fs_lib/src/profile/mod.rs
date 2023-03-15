pub mod models;

use models::*;
use diesel::prelude::*;
use crate::db_connect;
use crate::error::{Error, Result};
use super::gateway::models::Gateway;

pub fn all_profiles() -> Result<Vec<Profile>> {
    use crate::schema::profiles::dsl::*;
    let mut conn = db_connect();

    let profs = profiles
        .load::<Profile>(&mut conn)?;

    Ok(profs)
}

pub fn get_profile_id_by(profile_name: &str) -> Result<i32> {
    use crate::schema::profiles::dsl::*;

    let mut conn = db_connect();
    let ids = profiles
        .filter(name.eq(profile_name))
        .limit(1)
        .load::<Profile>(&mut conn)?;
    if ids.len() == 0 {
       return Err(Error::Fslib("Profile doesn't exist".to_string()));
    }

    Ok(ids[0].id)
}

pub fn profile_params(n: String) -> Result<Vec<ProfileParam>> {
    use crate::schema::profile_params::dsl::*;

    let mut conn = db_connect();
    let prof_id = get_profile_id_by(&n)?;

    let results = profile_params
        .filter(profile_id.eq(prof_id))
        .load::<ProfileParam>(&mut conn)?;

    Ok(results)
}

pub fn gateways(profile_id: i32) -> Result<Vec<Gateway>> {
    use crate::schema::profiles::dsl::*;

    let mut conn = db_connect();
    let prof = profiles.find(profile_id).get_result::<Profile>(&mut conn)?;
    let gateways = Gateway::belonging_to(&prof).load(&mut conn)?;

    Ok(gateways)

}
