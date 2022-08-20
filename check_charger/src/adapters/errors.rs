use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseFloatError;
use aws_sdk_dynamodb::error::ScanError;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::SdkError;
use crate::adapters::AdapterError::{DatabaseError, HttpError, ParseError};

#[derive(Debug)]
pub enum AdapterError {
    ParseError,
    DatabaseError,
    HttpError,
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
        HttpError
    }
}

impl From<SdkError<ScanError>> for AdapterError {
    fn from(_: SdkError<ScanError>) -> Self {
        DatabaseError
    }
}
