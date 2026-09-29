use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Database error")]
    Database(#[from] diesel::result::Error),

    #[error("FS library error")]
    Fslib(String),
}

pub type Result<T> =  std::result::Result<T, Error>;
