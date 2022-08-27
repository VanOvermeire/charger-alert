use lambda_http::{Request, RequestExt};
use query_map::QueryMap;
use serde::{Deserialize};
use common::{Coordinate, NorthEastLatitude, NorthEastLongitude, SouthWestLatitude, SouthWestLongitude};
use crate::adapters::AdapterError;

#[derive(Deserialize, Debug)]
pub struct GetChargersRequest {
    pub ne_lat: NorthEastLatitude,
    pub ne_lon: NorthEastLongitude,
    pub sw_lat: SouthWestLatitude,
    pub sw_lon: SouthWestLongitude,
}

impl TryInto<GetChargersRequest> for Request {
    type Error = AdapterError;

    fn try_into(self) -> Result<GetChargersRequest, Self::Error> {
        let params = self.query_string_parameters();

        let ne_lat: NorthEastLatitude = from_query_params(&params, "ne_lat")?;
        let ne_lon: NorthEastLongitude = from_query_params(&params, "ne_lon")?;
        let sw_lat: SouthWestLatitude = from_query_params(&params, "sw_lat")?;
        let sw_lon: SouthWestLongitude = from_query_params(&params, "sw_lon")?;

        Ok(GetChargersRequest {
            ne_lat,
            ne_lon,
            sw_lat,
            sw_lon,
        })
    }
}

// I'd like to use a closure here
// "let from_query_params_for_query_map = |name| from_query_params(&params, name);"
// but that only works for ONE coordinate type. the others try to get the same type
// can't seem to make the closure generic, and specifying coordinate as return means I can't just give the return values to the request
// this issue with type inference is similar to the one in database.rs, where I wanted to iter over the inputs
fn from_query_params<C: Coordinate>(params: &QueryMap, name: &str) -> Result<C, AdapterError> {
    let param = params.first(name);
    Ok(param.ok_or_else(|| AdapterError::InputError)
        .and_then(|v| v.parse::<f32>().map_err(|_| AdapterError::InputError))
        .map(C::new)?)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn should_turn_a_http_request_into_a_get_chargers_request() {
        let params = HashMap::from([
            ("ne_lat".to_string(), vec!["2.3".to_string()]),
            ("ne_lon".to_string(), vec!["1.5".to_string()]),
            ("sw_lon".to_string(), vec!["12.8".to_string()]),
            ("sw_lat".to_string(), vec!["55".to_string()]),
        ]);
        let request = Request::default().with_query_string_parameters(QueryMap::from(params));

        let result: GetChargersRequest = request.try_into().expect("try_into to succeed for request");

        assert_eq!(result.ne_lat.0, 2.3);
        assert_eq!(result.ne_lon.0, 1.5);
        assert_eq!(result.sw_lon.0, 12.8);
        assert_eq!(result.sw_lat.0, 55.0);
    }

    #[test]
    fn should_turn_an_http_request_with_missing_value_into_an_error() {
        let params = HashMap::from([
            ("ne_lon".to_string(), vec!["1.5".to_string()]),
            ("sw_lon".to_string(), vec!["12.8".to_string()]),
            ("sw_lat".to_string(), vec!["55".to_string()]),
        ]);
        let request = Request::default().with_query_string_parameters(QueryMap::from(params));

        let result: Result<GetChargersRequest, AdapterError> = request.try_into();

        assert!(result.is_err());
    }
}