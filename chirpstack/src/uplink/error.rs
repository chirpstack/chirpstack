use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Nothing else to do")]
    Abort,

    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
}
