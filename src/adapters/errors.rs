use std::env::VarError;
use std::error::Error;
use std::fmt::{Display, Formatter};
use aws_sdk_dynamodb::SdkError;
use lambda_http::ext::PayloadError;
use lambda_http::{Response};

// all adapter errors //

#[derive(Debug)]
pub enum AdapterError {
    ConfigError,
    InputError,
    DatabaseError,
}

impl Display for AdapterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AdapterError::InputError => write!(f, "Invalid input"),
            _ => write!(f, "Internal server error"),
        }
    }
}

impl From<VarError> for AdapterError {
    fn from(_: VarError) -> Self {
        AdapterError::ConfigError
    }
}

impl From<PayloadError> for AdapterError {
    fn from(_: PayloadError) -> Self {
        AdapterError::InputError
    }
}

impl<E> From<SdkError<E>> for AdapterError {
    fn from(_: SdkError<E>) -> Self {
        AdapterError::DatabaseError
    }
}

impl Error for AdapterError {}

impl AdapterError {
    pub fn to_response(&self) -> lambda_http::http::Result<Response<&'static str>> {
        match self {
            AdapterError::InputError => Response::builder().status(400).body("Invalid input"),
            AdapterError::ConfigError => Response::builder().status(500).body("Internal server error"),
            AdapterError::DatabaseError => Response::builder().status(500).body("Internal server error"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_400_response_for_an_input_error() {
        let result = AdapterError::InputError.to_response().expect("to_response to contain a response for input error");

        assert_eq!(result.status(), 400);
        assert_eq!(result.body().to_string(), "Invalid input".to_string());
    }

    #[test]
    fn should_return_500_response_for_a_config_error() {
        let result = AdapterError::ConfigError.to_response().expect("to_response to contain a response for config error");

        assert_eq!(result.status(), 500);
        assert_eq!(result.body().to_string(), "Internal server error".to_string());
    }

    #[test]
    fn should_return_500_response_for_a_database_error() {
        let result = AdapterError::DatabaseError.to_response().expect("to_response to contain a response for config error");

        assert_eq!(result.status(), 500);
        assert_eq!(result.body().to_string(), "Internal server error".to_string());
    }
}
