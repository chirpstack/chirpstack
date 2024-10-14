use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Nothing else to do")]
    Abort,

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error(transparent)]
    Storage(#[from] crate::storage::error::Error),
}
