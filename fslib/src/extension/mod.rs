use crate::schema::{extension};
use diesel::prelude::*;
use diesel::dsl::*;
use crate::db_connect;
use crate::error::{Result, Error};

#[derive(Queryable, Debug)]
pub struct Extension {
    pub id: i32,
    pub exten: String,
    pub exten_type: String,
    pub domain_id: i32
}

#[derive(Insertable)]
#[diesel(table_name=extension)]
pub struct NewExtension<'a> {
    pub exten: &'a str,
    pub exten_type: &'a str,
    pub domain_id: i32
}

#[derive(Queryable, Debug)]
pub struct ExtensionType {
    pub id:i32,
    pub name: String
}

pub fn add_extension(exten :&str, exten_type: &str, domain_id: i32) -> Result<()>{
    let mut conn = db_connect();
    let new_extension = NewExtension {
        exten, exten_type, domain_id
    };

    diesel::insert_into(extension::table)
        .values(&new_extension)
        .execute(&mut conn)?;

    Ok(())
}

pub fn del_extension(ext: &str) -> Result<()>{
    use crate::schema::extension::columns::*;

    let mut conn = db_connect();
    diesel::delete(extension::table)
        .filter(exten.eq(ext))
        .execute(&mut conn)?;

    Ok(())
}

pub fn ls_extension() -> Result<Vec<Extension>> {
    use crate::schema::extension::dsl::*;
    let mut conn = db_connect();
    let result = extension
        .load::<Extension>(&mut conn)?;

    Ok(result)
}

pub fn get_extension(e: &str, d_id: i32) -> Result<Extension> {
    use crate::schema::extension::dsl::*;
    let mut conn = db_connect();

    let mut result = extension
        .filter(exten.eq(e))
        .filter(domain_id.eq(d_id))
        .load::<Extension>(&mut conn)?;

    match result.pop() {
        Some(e) => {
            Ok(e)
        },
        None => {
            Err(Error::Fslib("Extension doesn't exist".to_string()))
        }
    }
}

pub fn type_exists(n: &str) -> Result<bool> {
    use crate::schema::extension_type::dsl::*;
    let mut conn = db_connect();

    let exists = select(exists(extension_type.filter(name.eq(n)))).get_result::<bool>(&mut conn)?;
    return Ok(exists);
}
