use std::env;
use aws_sdk_dynamodb::model::AttributeValue;
use lambda_http::{Request, RequestExt};
use serde::{Deserialize};
use common::{Lat, Lon};
use crate::adapters::AdapterError;

#[derive(Deserialize, Debug)]
pub struct ChargerRequest {
    lat: Lat,
    lon: Lon,
}

impl ChargerRequest {
    pub fn get_lat_and_long(&self) -> (&Lat, &Lon) {
        (&self.lat, &self.lon)
    }
}

impl TryInto<ChargerRequest> for Request {
    type Error = AdapterError;

    fn try_into(self) -> Result<ChargerRequest, Self::Error> {
        self.payload()?
            .ok_or(AdapterError::InputError)
    }
}

#[cfg(test)]
mod tests {
    use lambda_http::Body;
    use lambda_http::http::header::CONTENT_TYPE;
    use super::*;

    #[test]
    fn should_turn_a_http_request_into_a_charger_request() {
        let body_string = r#"{ "lat": 2.3, "lon": 1.5 }"#;
        let mut request = Request::new(Body::Text(body_string.to_owned()));
        let headers = request.headers_mut();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let result: ChargerRequest = request.try_into().expect("try_into to succeed for request");

        assert_eq!(result.get_lat_and_long().0.0, 2.3);
        assert_eq!(result.get_lat_and_long().1.0, 1.5);
    }

    #[test]
    fn should_turn_an_http_request_without_json_content_type_into_an_error() {
        let body_string = r#"{ "lat": 2.3, "lon": 1.5 }"#;
        let mut request = Request::new(Body::Text(body_string.to_owned()));
        let headers = request.headers_mut();
        headers.insert(CONTENT_TYPE, "text".parse().unwrap());

        let result: Result<ChargerRequest, AdapterError> = request.try_into();

        assert!(result.is_err());
    }

    #[test]
    fn should_turn_an_http_request_with_missing_value_into_an_error() {
        let body_string = r#"{ "lat": 2.3 }"#;
        let mut request = Request::new(Body::Text(body_string.to_owned()));
        let headers = request.headers_mut();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let result: Result<ChargerRequest, AdapterError> = request.try_into();

        assert!(result.is_err());
    }

    #[test]
    fn should_turn_an_invalid_http_request_into_an_error() {
        let body_string_with_missing_ending_bracket = r#"{ "lat": 2.3, "lon": 1.5"#;
        let mut request = Request::new(Body::Text(body_string_with_missing_ending_bracket.to_owned()));
        let headers = request.headers_mut();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let result: Result<ChargerRequest, AdapterError> = request.try_into();

        assert!(result.is_err());
    }
}