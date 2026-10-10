pub mod models;

use std::process::Command;
use std::env::temp_dir;
use std::path::Path;
use uuid::Uuid;

use models::*;
use diesel::prelude::*;
use crate::db_connect;
use crate::schema::recordings;
use crate::fs;
use crate::error::{Result, Error};

pub fn import(domain_id: i32, path: String) -> Result<()> {
    let mut conn = db_connect();
    let parts: Vec<&str> = path.split('/').collect();
    let recording_dir = fs::eval("$${sounds_dir}");
    let full_path = format!("{}/en/us/callie/{}/8000/{}", recording_dir, parts[0], parts[1]);

    if !Path::new(&full_path).exists() {
        return Err(Error::Fslib("Import file dosen't exist".to_string()));
    }

    let recording = NewRecording {
        name: &path,
        domain_id: domain_id,
        description: Some("FS recording imported"),
    };

    diesel::insert_into(recordings::table)
        .values(&recording)
        .execute(&mut conn)?;

    Ok(())
}

pub fn add(domain: i32,name: String, path: String, desc: String) -> Result<()>{
    let mut conn = db_connect();
    let name = format!("{}-{}.wav", name, domain);

    match install_recording(&path, &name) {
        Ok(()) => {
            let new_recording = NewRecording {
                name: &name,
                domain_id: domain,
                description: Some(&desc)
            };
            diesel::insert_into(recordings::table)
                .values(&new_recording)
                .execute(&mut conn)?;
        },
        Err(e) => {
            println!("{:?}", e);
            return Err(Error::Fslib("Install recording failed".to_string()));
        }
    }

    Ok(())
}

pub fn list() -> Result<Vec<Recording>>{
    use crate::schema::recordings::dsl::*;

    let mut conn = db_connect();
    let result = recordings
        .load::<Recording>(&mut conn)?;

    Ok(result)
}

pub fn del(a_id: i32) -> Result<()>{
    use crate::schema::recordings::columns::id;

    let mut conn = db_connect();
    diesel::delete(recordings::table)
        .filter(id.eq(a_id))
        .execute(&mut conn)?;

    Ok(())
}


pub fn get(a_id: i32) -> Result<Recording> {
    use crate::schema::recordings::dsl::*;
    let mut conn = db_connect();

    let result = recordings
        .find(a_id)
        .first(&mut conn)?;

    Ok(result)
}

pub fn update(r: Recording) -> Result<()> {
    use crate::schema::recordings;
    use crate::schema::recordings::dsl::*;

    let mut conn = db_connect();
    let mut pre = get(r.id)?;

    pre.description = r.description.clone();
    diesel::update(recordings::table)
        .filter(id.eq(r.id))
        .set(&pre)
        .execute(&mut conn)?;

    Ok(())
}
fn make_tmp_name() -> String {
    let tmp_dir = temp_dir();
    let file_name = format!("{}/{}.wav",
                            tmp_dir.to_str().unwrap(),
                            Uuid::new_v4());

    file_name
}

fn install_recording(path: &str, name: &str) -> Result<()> {
    let tmp_file = make_tmp_name();
    let recording_dir = fs::eval("$${sounds_dir}");
    let target_path = format!("{}/en/us/callie/{}", recording_dir, name);

    let mut status = Command::new("mpg123")
        .arg("-w")
        .arg(&tmp_file)
        .arg(path)
        .status()
        .unwrap();

    if !status.success() {
        return Err(Error::Fslib("Add recording failed".to_string()));
    }

    status = Command::new("sox")
        .arg(&tmp_file)
        .arg("-c")
        .arg("1")
        .arg("-r")
        .arg("8000")
        .arg(target_path)
        .status()
        .unwrap();

    if !status.success() {
        return Err(Error::Fslib("Add recording failed".to_string()));
    }

    status = Command::new("rm")
        .arg("-f")
        .arg(&tmp_file)
        .status()
        .unwrap();

    if !status.success() {
        return Err(Error::Fslib("Add recording failed".to_string()));
    }

    return Ok(())
}
