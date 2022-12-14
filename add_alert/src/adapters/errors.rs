use std::error::Error;
use std::fmt::{Display, Formatter};
use lambda_http::ext::PayloadError;
use lambda_http::{Response};
use common::{bad_request_response, internal_server_error_response};

#[derive(Debug)]
pub enum AdapterError {
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

impl From<PayloadError> for AdapterError {
    fn from(_: PayloadError) -> Self {
        AdapterError::InputError
    }
}

impl Error for AdapterError {}

impl AdapterError {
    pub fn to_http_response(&self) -> lambda_http::http::Result<Response<String>> {
        match self {
            AdapterError::InputError => bad_request_response("Invalid input"),
            AdapterError::DatabaseError => internal_server_error_response(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_400_response_for_an_input_error() {
        let result = AdapterError::InputError.to_http_response().expect("to_response to contain a response for input error");

        assert_eq!(result.status(), 400);
        assert_eq!(result.body().to_string(), "Invalid input".to_string());
    }

    #[test]
    fn should_return_500_response_for_a_database_error() {
        let result = AdapterError::DatabaseError.to_http_response().expect("to_response to contain a response for config error");

        assert_eq!(result.status(), 500);
        assert_eq!(result.body().to_string(), "Internal server error".to_string());
    }
}
