use std::env;
use lambda_http::{Request, RequestExt};
use serde::{Deserialize};
use crate::adapters::AdapterError;

const REGION_KEY: &'static str = "REGION";
const TABLE_KEY: &'static str = "TABLE";

#[derive(Debug, Clone)]
pub struct Region(pub String); // TODO instead provide a 'into'?
#[derive(Debug, Clone)]
pub struct Table(pub String);

#[derive(Debug, Clone)]
pub struct Config {
    table: Table,
    region: Region,
}

impl Config {
    pub fn new() -> Result<Config, AdapterError> {
        let region = env::var(REGION_KEY)?;
        let table = env::var(TABLE_KEY)?;

        Ok(Config {
            table: Table(table),
            region: Region(region),
        })
    }

    pub fn get_table(&self) -> &Table {
        &self.table
    }

    pub fn get_region(&self) -> &Region {
        &self.region
    }
}

#[derive(Deserialize, Debug)]
pub struct Lat(pub f32);
#[derive(Deserialize, Debug)]
pub struct Lon(pub f32);

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