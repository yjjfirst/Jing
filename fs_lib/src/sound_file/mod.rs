pub mod models;

use std::process::Command;
use std::env::temp_dir;
use uuid::Uuid;

use models::*;
use diesel::prelude::*;
use crate::db_connect;
use crate::schema::sound_files;
use crate::rt;
use crate::error::{Result, Error};

pub fn add(domain: i32,name: String, path: String, desc: String) -> Result<()>{
    let mut conn = db_connect();
    let name = format!("{}-{}.wav", name, domain);

    match install_file(&path, &name) {
        Ok(()) => {
            let new_soundfile = NewSoundFile {
                name: &name,
                domain_id: domain,
                description: Some(&desc)
            };
            diesel::insert_into(sound_files::table)
                .values(&new_soundfile)
                .execute(&mut conn)?;
        },
        Err(e) => {
            println!("{:?}", e);
            return Err(Error::Fslib("Install sound file failed".to_string()));
        }
    }

    Ok(())
}

pub fn list() -> Result<Vec<SoundFile>>{
    use crate::schema::sound_files::dsl::*;

    let mut conn = db_connect();
    let result = sound_files
        .load::<SoundFile>(&mut conn)?;

    Ok(result)
}

pub fn del(a_id: i32) -> Result<()>{
    use crate::schema::sound_files::columns::id;

    let mut conn = db_connect();
    diesel::delete(sound_files::table)
        .filter(id.eq(a_id))
        .execute(&mut conn)?;

    Ok(())
}


pub fn get(a_id: i32) -> Result<SoundFile> {
    use crate::schema::sound_files::dsl::*;
    let mut conn = db_connect();

    let result = sound_files
        .find(a_id)
        .first(&mut conn)?;

    Ok(result)
}

pub fn update(f: SoundFile) -> Result<()> {
    use crate::schema::sound_files;
    use crate::schema::sound_files::dsl::*;

    let mut conn = db_connect();
    let mut pre = get(f.id)?;

    pre.description = f.description.clone();
    diesel::update(sound_files::table)
        .filter(id.eq(f.id))
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

fn install_file(path: &str, name: &str) -> std::io::Result<()> {

    let tmp_file = make_tmp_name();
    let sound_dir = rt::eval("$${sounds_dir}");
    let target_path = format!("{}/en/us/callie/{}", sound_dir, name);

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
