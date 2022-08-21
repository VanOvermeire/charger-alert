use std::error::Error;
use std::fmt::{Display, Formatter};
use aws_sdk_dynamodb::types::SdkError;

#[derive(Debug)]
pub enum AdapterError {
    ParseError,
    DatabaseError,
    HttpError,
    EmailError,
}

impl Display for AdapterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "Internal server error"),
        }
    }
}

impl Error for AdapterError {}

impl From<reqwest::Error> for AdapterError {
    fn from(_: reqwest::Error) -> Self {
        AdapterError::HttpError
    }
}

impl <T>From<SdkError<T>> for AdapterError {
    fn from(_: SdkError<T>) -> Self {
        AdapterError::DatabaseError
    }
}
