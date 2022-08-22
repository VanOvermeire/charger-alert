use std::sync::Arc;

use futures::future::join_all;
use lambda_runtime::{Error, LambdaEvent, service_fn};
use serde_json::{json, Value};

use common::{build_db_client, ChargerLambdaConfig, Email};

use crate::adapters::{AdapterError, build_email_client, build_http_client, CoordinatesDatabase, DbId, EmailClient, HttpClient};

mod adapters;

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

async fn flow<T: CoordinatesDatabase>(config: Arc<ChargerLambdaConfig>, db_client: Arc<T>, http_client: Arc<HttpClient>, email_client: Arc<EmailClient>) -> Result<Value, Error> {
    for item in db_client.get(config.get_table().0.as_str()).await? {
        let chargers = http_client.get_chargers(item.ne_lat, item.ne_lon, item.sw_lat, item.sw_lon).await?;
        let email_and_db_results: Vec<Result<(), AdapterError>> = join_all(chargers.iter()
            .filter(|c| c.available_connectors > 0)
            .map(|c| async {
                println!("Charger with id {} has available connectors!", c.id);
                send_email_and_delete_item(&item.id, &item.email,config.clone(), db_client.clone(), email_client.clone()).await
            })
            .collect::<Vec<_>>()).await;
        let _ = email_and_db_results.into_iter().collect::<Result<Vec<()>, AdapterError>>()?;
    };

    Ok(json!(
        { "message": "done" }
    ))
}

async fn send_email_and_delete_item<T: CoordinatesDatabase>(id: &DbId, email: &Email, config: Arc<ChargerLambdaConfig>, db_client: Arc<T>, email_client: Arc<EmailClient>) -> Result<(), AdapterError> {
    email_client.send(email).await?;
    Ok(db_client.delete(config.get_table().0.as_str(), id).await?)
}
