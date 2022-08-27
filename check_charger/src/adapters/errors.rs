use std::error::Error;
use std::fmt::{Display, Formatter};
use aws_sdk_dynamodb::error::{DeleteItemError, ScanError};
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::types::SdkError;
use aws_sdk_ses::error::SendEmailError;

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

impl From<SdkError<ScanError>> for AdapterError {
    fn from(_: SdkError<ScanError>) -> Self {
        AdapterError::DatabaseError
    }
}

impl From<SdkError<DeleteItemError>> for AdapterError {
    fn from(_: SdkError<DeleteItemError>) -> Self {
        AdapterError::DatabaseError
    }
}

impl From<SdkError<SendEmailError>> for AdapterError {
    fn from(_: SdkError<SendEmailError>) -> Self {
        AdapterError::EmailError
    }
}

impl From<&AttributeValue> for AdapterError {
    fn from(_: &AttributeValue) -> Self {
        AdapterError::ParseError
    }
}