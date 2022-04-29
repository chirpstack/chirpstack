use diesel::result::Error as ResultError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Object does not exist (id: {0})")]
    NotFound(String),

    #[error("Object already exists (id: {0})")]
    AlreadyExists(String),

    #[error("Invalid email address format")]
    InvalidEmail,

    #[error("Hash password error (error: {0})")]
    HashPassword(String),

    #[error("Invalid username or password")]
    InvalidUsernameOrPassword,

    #[error("Invalid type (expected: {0})")]
    InvalidPayload(String),

    #[error("Invalid MIC")]
    InvalidMIC,

    #[error("Invalid DevNonce")]
    InvalidDevNonce,

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not allowed ({0})")]
    NotAllowed(String),

    #[error(transparent)]
    Diesel(#[from] diesel::result::Error),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error(transparent)]
    Lrwn(#[from] lrwn::Error),

    #[error(transparent)]
    TokioJoin(#[from] tokio::task::JoinError),

    #[error(transparent)]
    Redis(#[from] redis::RedisError),

    #[error(transparent)]
    ProstDecode(#[from] prost::DecodeError),
}

impl Error {
    pub fn from_diesel(e: diesel::result::Error, s: String) -> Self {
        match &e {
            ResultError::NotFound => Error::NotFound(s),
            _ => Error::Diesel(e),
        }
    }
}
