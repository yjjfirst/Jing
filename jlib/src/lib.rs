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
pub mod sound_file;
pub mod sound;
pub mod conference;
pub mod callcenter;
pub mod acl;
pub mod feature_code;
pub mod printable;
pub mod portal_user;
pub mod portal_token;
pub mod system_setting;
pub mod firewall;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate util_macro;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use rand::Rng;
use dotenv::dotenv;
use std::env;

pub fn db_connect() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn generate_token(length: usize, options: Option<(bool, bool, bool)>) -> Result<String, &'static str> {
    let (uppercase, lowercase, numbers) = match options {
        Some((uppercase, lowercase, numbers)) => (uppercase, lowercase, numbers),
        None => (true, true, true),
    };

    let mut charset = String::new();
    if uppercase {
        charset += "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    }
    if lowercase {
        charset += "abcdefghijklmnopqrstuvwxyz";
    }
    if numbers {
        charset += "0123456789";
    }

    if charset.is_empty() {
        return Err("At least one character set must be selected.");
    }

    let mut rng = rand::thread_rng();
    let token: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect();

    Ok(token)
}
