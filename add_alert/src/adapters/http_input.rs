use lambda_http::{Request, RequestExt};
use serde::{Deserialize};
use common::{Email, NorthEastLatitude, NorthEastLongitude, SouthWestLatitude, SouthWestLongitude};
use crate::adapters::AdapterError;

#[derive(Deserialize, Debug)]
pub struct ChargerRequest {
    pub email: Email,
    pub ne_lat: NorthEastLatitude,
    pub ne_lon: NorthEastLongitude,
    pub sw_lat: SouthWestLatitude,
    pub sw_lon: SouthWestLongitude,
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
        let body_string = r#"{ "ne_lat": 2.3, "ne_lon": 1.5, "sw_lat": 55, "sw_lon": 12.8, "email": "test@test.com" }"#;
        let mut request = Request::new(Body::Text(body_string.to_owned()));
        let headers = request.headers_mut();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let result: ChargerRequest = request.try_into().expect("try_into to succeed for request");

        assert_eq!(result.email.0, "test@test.com");
        assert_eq!(result.ne_lat.0, 2.3);
        assert_eq!(result.ne_lon.0, 1.5);
        assert_eq!(result.sw_lat.0, 55.0);
        assert_eq!(result.sw_lon.0, 12.8);
    }

    #[test]
    fn should_turn_an_http_request_without_json_content_type_into_an_error() {
        let body_string = r#"{ "ne_lat": 2.3, "ne_lon": 1.5, "sw_lat": 55, "sw_lon": 12.8, "email": "test@test.com" }"#;
        let mut request = Request::new(Body::Text(body_string.to_owned()));
        let headers = request.headers_mut();
        headers.insert(CONTENT_TYPE, "text".parse().unwrap());

        let result: Result<ChargerRequest, AdapterError> = request.try_into();

        assert!(result.is_err());
    }

    #[test]
    fn should_turn_an_http_request_with_missing_value_into_an_error() {
        let body_string = r#"{ "ne_lat": 2.3, "sw_lat": 55, "sw_lon": 12.8, "email": "test@test.com" }"#;
        let mut request = Request::new(Body::Text(body_string.to_owned()));
        let headers = request.headers_mut();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let result: Result<ChargerRequest, AdapterError> = request.try_into();

        assert!(result.is_err());
    }

    #[test]
    fn should_turn_an_invalid_http_request_into_an_error() {
        let body_string_with_missing_ending_bracket = r#"{ "ne_lat": 2.3, "sw_lat": 55, "sw_lon": 12.8, "email": "test@test.com""#;
        let mut request = Request::new(Body::Text(body_string_with_missing_ending_bracket.to_owned()));
        let headers = request.headers_mut();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let result: Result<ChargerRequest, AdapterError> = request.try_into();

        assert!(result.is_err());
    }
}