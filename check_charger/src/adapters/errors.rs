use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseFloatError;
use aws_sdk_dynamodb::error::ScanError;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::SdkError;
use crate::adapters::AdapterError::{DatabaseError, ParseError};

#[derive(Debug)]
pub enum AdapterError {
    ParseError,
    DatabaseError,
}

impl Display for AdapterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "Internal server error"),
        }
    }
}

impl Error for AdapterError {}

// impl From<&AttributeValue> for AdapterError {
//     fn from(_: &AttributeValue) -> Self {
//         ParseError
//     }
// }
//
// impl From<ParseFloatError> for AdapterError {
//     fn from(_: ParseFloatError) -> Self {
//         ParseError
//     }
// }

impl From<SdkError<ScanError>> for AdapterError {
    fn from(_: SdkError<ScanError>) -> Self {
        DatabaseError
    }
}
