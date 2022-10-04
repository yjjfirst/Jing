pub mod models;

use std::process::Command;
use std::env::temp_dir;
use uuid::Uuid;

use models::*;
use diesel::prelude::*;
use crate::db_connect;
use crate::schema::sound_file;
use crate::domain;
use crate::rt;
use crate::error::{Result, Error};

pub fn add(name: String, path: String, desc: String) -> Result<()>{
    let domain = domain::get_active().unwrap();
    let conn = db_connect();
    let name = format!("{}-{}", name, domain.id);

    match install_file(&path, &name) {
        Ok(()) => {
            let new_sound = NewSoundFile {
                name: &name,
                domain_id: domain.id,
                desc: Some(&desc)
            };
            diesel::insert_into(sound_file::table)
                .values(&new_sound)
                .execute(&conn)?;
        },
        Err(_) => {
            return Err(Error::Fslib("Add sound failed".to_string()));
        }
    }

    Ok(())
}

pub fn all() -> Result<Vec<SoundFile>>{
    use crate::schema::sound_file::dsl::*;

    let conn = db_connect();
    let result = sound_file
        .load::<SoundFile>(&conn)?;

    Ok(result)
}

pub fn del(a_id: i32) -> Result<()>{
    use crate::schema::sound_file::columns::id;

    let conn = db_connect();
    diesel::delete(sound_file::table)
        .filter(id.eq(a_id))
        .execute(&conn)?;

    Ok(())
}


fn make_tmp_name() -> String {
    let tmp_dir = temp_dir();
    let file_name = format!("{}/{}.wav",
                            tmp_dir.to_str().unwrap(),
                            Uuid::new_v4());

    file_name
}

fn install_file(path: &str, name: &str) -> std::io::Result<()> {

    let tmp_file = make_tmp_name();
    let sound_dir = rt::eval("$${sounds_dir}");
    let domain = domain::get_active().unwrap();
    let target_path = format!("{}/{}-{}.wav", sound_dir, name, domain.id);

    Command::new("mpg123")
        .arg("-w")
        .arg(&tmp_file)
        .arg(path)
        .status()?;

    Command::new("sox")
        .arg(&tmp_file)
        .arg("-c")
        .arg("1")
        .arg("-r")
        .arg("8000")
        .arg(target_path)
        .status()?;

    Command::new("rm")
        .arg("-f")
        .arg(&tmp_file)
        .status()?;

    return Ok(())
}
