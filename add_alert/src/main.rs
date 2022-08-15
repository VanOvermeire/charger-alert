mod adapters;

use std::sync::Arc;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, Region};
use std::convert::TryInto;
use lambda_http::{Body, Request, Response, service_fn};
use crate::adapters::{ChargerRequest, Config, Database, DynamoDB, success_response};

// TODO use Database trait instead of dynamodb?

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    let lambda_config = Arc::new(
        Config::new().expect("Config to be available")
    );
    let db_client = build_db_client(lambda_config.clone().as_ref().get_region()).await;

    lambda_http::run(service_fn(move |r: Request| {
        fun(r, lambda_config.clone(), db_client.clone())
    })).await?;

    Ok(())
}

async fn fun(request: Request, config: Arc<Config>, arc_client: Arc<DynamoDB>) -> lambda_http::http::Result<Response<String>> {
    flow(request, config, arc_client).await
}

async fn flow(request: Request, config: Arc<Config>, arc_client: Arc<DynamoDB>) -> lambda_http::http::Result<Response<String>> {
    match <lambda_http::http::Request<Body> as TryInto<ChargerRequest>>::try_into(request) {
        Ok(req) => {
            let lat_and_lon = req.get_lat_and_long();
            match arc_client.as_ref().add_lat_and_lon(config.as_ref().get_table().0.as_ref(), lat_and_lon.0, lat_and_lon.1).await {
                Ok(_) => success_response(),
                Err(e) => e.to_http_response(),
            }
        },
        Err(e) => {
            return e.to_http_response()
        }
    }
}

async fn build_db_client(region: &adapters::Region) -> Arc<DynamoDB> {
    let region_provider = RegionProviderChain::first_try(Region::new(region.0.clone())).or_default_provider();
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    Arc::new(DynamoDB::new(Client::new(&shared_config)))
}
