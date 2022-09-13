mod adapters;

use std::convert::TryInto;
use std::rc::Rc;
use lambda_http::{Body, Request, Response, service_fn};
use crate::adapters::{ChargerRequest, CoordinatesDb};
use common::{build_db_client, ChargerLambdaConfig, success_response};

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    let lambda_config = Rc::new(
        ChargerLambdaConfig::new().expect("Config to be available")
    );
    let db_client = build_db_client(lambda_config.clone().as_ref().get_region()).await;

    lambda_http::run(service_fn(move |r: Request| {
        flow(r, lambda_config.clone(), db_client.clone())
    })).await
}

// uses trait instead of a specific implementation - easier to switch out
async fn flow<T: CoordinatesDb>(request: Request, config: Rc<ChargerLambdaConfig>, arc_client: Rc<T>) -> lambda_http::http::Result<Response<String>> {
    match <lambda_http::http::Request<Body> as TryInto<ChargerRequest>>::try_into(request) {
        Ok(req) => {
            match arc_client.add(config.get_table().0.as_ref(), req.email, req.charger_id, req.ne_lat, req.ne_lon, req.sw_lat, req.sw_lon).await {
                Ok(_) => success_response(),
                Err(e) => e.to_http_response(),
            }
        },
        Err(e) => {
            return e.to_http_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use async_trait::async_trait;
    use lambda_http::http::header::CONTENT_TYPE;
    use common::{ChargerId, Email, NorthEastLatitude, NorthEastLongitude, SouthWestLatitude, SouthWestLongitude};
    use crate::adapters::AdapterError;
    use super::*;

    struct FakeDB {}

    #[async_trait]
    impl CoordinatesDb for FakeDB {
        async fn add(&self, table: &str, _email: Email, charger_id: ChargerId, _ne_lat: NorthEastLatitude, _ne_lon: NorthEastLongitude, _sw_lat: SouthWestLatitude, _sw_lon: SouthWestLongitude) -> Result<(), AdapterError> {
            assert_eq!(charger_id.0, 5);
            assert_eq!(table, "fake-table");

            Ok(())
        }
    }

    #[tokio::test]
    async fn flow_should_work_for_valid_request() {
        // configure env
        env::set_var("REGION", "fake-west-1");
        env::set_var("TABLE", "fake-table");

        let request = build_request();
        let config = Rc::new(ChargerLambdaConfig::new().expect("Test config to be created"));
        let db = Rc::new(FakeDB {});

        let result = flow(request, config, db).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().status(), 200);
    }

    fn build_request() -> Request {
        let body_string = r#"{ "ne_lat": 2.3, "ne_lon": 1.5, "sw_lat": 55, "sw_lon": 12.8, "email": "test@test.com", "charger_id": 5 }"#;
        let mut request = Request::new(Body::Text(body_string.to_owned()));
        let headers = request.headers_mut();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        request
    }
}
