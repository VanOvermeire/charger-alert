mod adapters;

use std::sync::Arc;
use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{json, Value};
use common::{build_db_client, ChargerLambdaConfig};
use crate::adapters::{CoordinatesDatabase, HttpClient};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let lambda_config = Arc::new(
        ChargerLambdaConfig::new().expect("Config to be available")
    );
    let db_client = build_db_client(lambda_config.clone().as_ref().get_region()).await;
    let http_client = Arc::new(HttpClient::default());

    lambda_runtime::run(service_fn(move |_: LambdaEvent<Value>| {
        flow(lambda_config.clone(), db_client.clone(), http_client.clone())
    })).await
}

async fn flow<T: CoordinatesDatabase>(config: Arc<ChargerLambdaConfig>, arc_client: Arc<T>, http_client: Arc<HttpClient>) -> Result<Value, Error> {
    for item in arc_client.as_ref().get(config.get_table().0.as_str()).await? {
        //  - if we find any matches with free connectors
        //  - then send an SES message (configure env vars for this)
        //  - and delete the entry

        // http_client.as_ref().get(item.ne_lat, item.ne_lon, item.sw_lat, item.sw_lon).await;
    };

    Ok(json!(
        { "message": "done" }
    ))
}
