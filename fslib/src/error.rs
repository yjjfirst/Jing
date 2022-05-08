use thiserror::Error;

#[derive(Error, Debug)]
pub enum HornetError {
    #[error("Dest type doesn't exist")]
    DestNonExist,
    
    #[error("Profile doesn't exist")]
    ProfileNonExist,

    #[error("Database error")]
    Database(#[from] diesel::result::Error),

    #[error("Aleg doesn't exist")]
    AlegNonExist,
    
    #[error("Logic error")]
    LogicError(String),
}

pub type Result<T> =  std::result::Result<T, HornetError>;

