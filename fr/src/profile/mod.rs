pub mod models;

use models::*;
use diesel::prelude::*;
use crate::db_connect;
use crate::error::{HornetError, Result};

pub fn all_profiles() -> Result<Vec<Profile>> {
    use crate::schema::profile::dsl::*;
    let conn = db_connect();

    let profiles = profile
        .load::<Profile>(&conn)?;
    
    Ok(profiles)
}

pub fn get_profile_id_by(profile_name: &str) -> Result<i32> {
    use crate::schema::profile::dsl::*;

    let conn = db_connect();
    let ids = profile
        .filter(name.eq(profile_name))
        .limit(1)
        .load::<Profile>(&conn)?;
    if ids.len() == 0 {
       return Err(HornetError::ProfileNonExist);
    }

    Ok(ids[0].id)
}

pub fn profile_params(n: String) -> Result<Vec<ProfileParam>> {
    use crate::schema::profile_param::dsl::*;

    let conn = db_connect();
    let prof_id = get_profile_id_by(&n)?;
    
    let results = profile_param
        .filter(profile_id.eq(prof_id))
        .load::<ProfileParam>(&conn)?;

    Ok(results)
}
