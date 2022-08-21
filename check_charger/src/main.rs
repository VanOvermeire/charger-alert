mod adapters;

use std::future::Future;
use futures::stream::{self, StreamExt};
use std::sync::Arc;
use futures::future::join_all;
use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{json, Value};
use common::{build_db_client, ChargerLambdaConfig};
use crate::adapters::{AdapterError, build_email_client, build_http_client, CoordinatesDatabase, EmailClient, HttpClient};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let lambda_config = Arc::new(
        ChargerLambdaConfig::new().expect("Config to be available")
    );
    let db_client = build_db_client(lambda_config.clone().as_ref().get_region()).await;
    let http_client = build_http_client().await;
    let email_client = build_email_client(
        lambda_config.clone().as_ref().get_email_address()
    ).await;

    lambda_runtime::run(service_fn(move |_: LambdaEvent<Value>| {
        flow(lambda_config.clone(), db_client.clone(), http_client.clone(), email_client.clone())
    })).await
}

async fn flow<T: CoordinatesDatabase>(config: Arc<ChargerLambdaConfig>, arc_client: Arc<T>, http_client: Arc<HttpClient>, email_client: Arc<EmailClient>) -> Result<Value, Error> {
    for item in arc_client.as_ref().get(config.get_table().0.as_str()).await? {
        let chargers = http_client.as_ref().get_chargers(item.ne_lat, item.ne_lon, item.sw_lat, item.sw_lon).await?;

        let t: Vec<Result<(), AdapterError>> = join_all(chargers.iter()
            .filter(|c| c.available_connectors > 0)
            .map(|c| async {
                println!("Charger with id {} has available connectors!", c.id);
                // TODO get email
                match email_client.send("FAKE&FAKE.com").await {
                    Ok(_) => {
                        println!("Email was sent, now deleting item in db");
                        arc_client.as_ref().delete(config.get_table().0.as_str(), &item.id).await
                    },
                    Err(e) => Err(e),
                }
            })
            .collect::<Vec<_>>()).await;
    };

    Ok(json!(
        { "message": "done" }
    ))
}
