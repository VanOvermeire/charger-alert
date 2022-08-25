use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct HttpError {}

impl Display for HttpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Internal server error")
    }
}

impl Error for HttpError {}

impl From<reqwest::Error> for HttpError {
    fn from(_: reqwest::Error) -> Self {
        HttpError {}
    }
}
