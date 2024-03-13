use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Nothing else to do")]
    Abort,

    #[error("Roaming is not allowed for the device")]
    RoamingIsNotAllowed,

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}
