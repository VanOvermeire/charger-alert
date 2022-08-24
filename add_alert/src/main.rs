mod adapters;

use std::convert::TryInto;
use std::rc::Rc;
use lambda_http::{Body, Request, Response, service_fn};
use crate::adapters::{ChargerRequest, CoordinatesDb, success_response};
use common::{build_db_client, ChargerLambdaConfig};

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
            match arc_client.add(config.get_table().0.as_ref(), req.email, req.ne_lat, req.ne_lon, req.sw_lat, req.sw_lon).await {
                Ok(_) => success_response(),
                Err(e) => e.to_http_response(),
            }
        },
        Err(e) => {
            return e.to_http_response()
        }
    }
}
