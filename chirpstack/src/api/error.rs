use tonic::{Code, Status};

use crate::storage;

pub trait ToStatus {
    fn status(&self) -> Status;
}

impl ToStatus for storage::error::Error {
    fn status(&self) -> Status {
        match self {
            storage::error::Error::NotFound(_) => Status::new(Code::NotFound, format!("{}", self)),
            storage::error::Error::AlreadyExists(_) => {
                Status::new(Code::AlreadyExists, format!("{}", self))
            }
            storage::error::Error::InvalidEmail => {
                Status::new(Code::InvalidArgument, format!("{}", self))
            }
            storage::error::Error::HashPassword(_) => {
                Status::new(Code::InvalidArgument, format!("{}", self))
            }
            storage::error::Error::InvalidUsernameOrPassword => {
                Status::new(Code::Unauthenticated, format!("{}", self))
            }
            storage::error::Error::InvalidPayload(_) => {
                Status::new(Code::Internal, format!("{}", self))
            }
            storage::error::Error::InvalidMIC => {
                Status::new(Code::InvalidArgument, format!("{}", self))
            }
            storage::error::Error::InvalidDevNonce => {
                Status::new(Code::InvalidArgument, format!("{}", self))
            }
            storage::error::Error::Validation(_) => {
                Status::new(Code::InvalidArgument, format!("{}", self))
            }
            storage::error::Error::NotAllowed(_) => {
                Status::new(Code::InvalidArgument, format!("{}", self))
            }
            storage::error::Error::Diesel(_) => Status::new(Code::Internal, format!("{}", self)),
            storage::error::Error::Anyhow(_) => Status::new(Code::Internal, format!("{}", self)),
            storage::error::Error::Lrwn(_) => Status::new(Code::Internal, format!("{}", self)),
            storage::error::Error::TokioJoin(_) => Status::new(Code::Internal, format!("{}", self)),
            storage::error::Error::Redis(_) => Status::new(Code::Internal, format!("{}", self)),
            storage::error::Error::ProstDecode(_) => {
                Status::new(Code::Internal, format!("{}", self))
            }
        }
    }
}

impl ToStatus for anyhow::Error {
    fn status(&self) -> Status {
        Status::new(Code::Internal, format!("{}", self))
    }
}

impl ToStatus for uuid::Error {
    fn status(&self) -> Status {
        Status::new(Code::InvalidArgument, format!("{}", self))
    }
}

impl ToStatus for r2d2::Error {
    fn status(&self) -> Status {
        Status::new(Code::Internal, format!("{}", self))
    }
}

impl ToStatus for lrwn::Error {
    fn status(&self) -> Status {
        Status::new(Code::Internal, format!("{}", self))
    }
}

impl ToStatus for Box<dyn std::error::Error> {
    fn status(&self) -> Status {
        Status::new(Code::Internal, format!("{}", self))
    }
}

impl ToStatus for tokio::task::JoinError {
    fn status(&self) -> Status {
        Status::new(Code::Internal, format!("{}", self))
    }
}

impl ToStatus for prost_types::TimestampError {
    fn status(&self) -> Status {
        Status::new(Code::Internal, format!("{}", self))
    }
}

impl ToStatus for std::num::ParseIntError {
    fn status(&self) -> Status {
        Status::new(Code::Internal, format!("{}", self))
    }
}
