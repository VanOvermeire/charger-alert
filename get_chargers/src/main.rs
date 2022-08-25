mod adapters;

use std::convert::TryInto;
use std::rc::Rc;
use lambda_http::{Body, Request, Response, service_fn};
use crate::adapters::{GetChargersRequest};
use common::{build_http_client, HttpClient, internal_server_error_response, success_response};

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    let http_client = build_http_client().await;
    lambda_http::run(service_fn(move |r: Request| {
        flow(r, http_client.clone())
    })).await
}

async fn flow(request: Request, http_client: Rc<HttpClient>) -> lambda_http::http::Result<Response<String>> {
    match <lambda_http::http::Request<Body> as TryInto<GetChargersRequest>>::try_into(request) {
        Ok(req) => {
            match http_client.get_chargers(req.ne_lat, req.ne_lon, req.sw_lat, req.sw_lon).await {
                Ok(_) => success_response(), // TODO return the charger info
                Err(_) => internal_server_error_response(),
            }
        },
        Err(e) => {
            return e.to_http_response()
        }
    }
}
