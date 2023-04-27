use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("DevAddrPrefix must be in the form 00000000/0")]
    DevAddrPrefixFormat,

    #[error("EuiPrefix must be in the form 0000000000000000/0")]
    EuiPrefixFormat,

    #[error("DevAddrPrefix max prefix size is 32")]
    DevAddrPrefixSize,

    #[error("EuiPrefix max prefix size is 64")]
    EuiPrefixSize,

    #[error(transparent)]
    FromHexError(#[from] hex::FromHexError),
}
