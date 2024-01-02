use std::net::AddrParseError;
use common::thiserror::Error;

#[derive(Debug, Error)]
pub enum PubError {
    #[error("{0}")]
    With(String),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    AddrParseError(#[from] AddrParseError),
}