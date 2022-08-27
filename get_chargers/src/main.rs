mod adapters;

use std::convert::TryInto;
use std::rc::Rc;
use lambda_http::{Body, Request, Response, service_fn};
use crate::adapters::{GetChargerOutput, GetChargersRequest};
use common::{build_http_client, HttpClient, internal_server_error_response, success_response_with_body};

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
                Ok(chargers) => {
                    let output_chargers: Vec<GetChargerOutput> = chargers.into_iter()
                        .map(|v| v.into())
                        .collect();
                    let response_body = serde_json::to_string(&output_chargers).expect("Should be able to convert charger output vec into json");
                    success_response_with_body(response_body)
                },
                Err(_) => internal_server_error_response(),
            }
        },
        Err(e) => {
            return e.to_http_response()
        }
    }
}
