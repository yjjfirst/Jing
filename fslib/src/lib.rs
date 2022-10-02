pub mod schema;
pub mod user;
pub mod profile;
pub mod gateway;
pub mod domain;
pub mod route;
pub mod error;
pub mod cdr;
pub mod voicemail;
pub mod ringgroup;
pub mod ivr;
pub mod extension;
pub mod rt;
pub mod sound;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

pub fn db_connect() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
