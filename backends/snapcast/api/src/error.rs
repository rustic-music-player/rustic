use thiserror::Error;
use crate::rpc::RpcError;

#[derive(Error, Debug)]
pub enum SnapcastError {
    #[error("http error {0}")]
    HttpError(String),
    #[error("rpc error {0}")]
    RpcError(#[from] RpcError)
}

pub type Result<T> = std::result::Result<T, SnapcastError>;

impl From<surf::Error> for SnapcastError {
    fn from(err: surf::Error) -> Self {
        SnapcastError::HttpError(err.to_string())
    }
}
