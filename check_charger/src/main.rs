use std::rc::Rc;

use lambda_runtime::{Error, LambdaEvent, service_fn};
use serde_json::{json, Value};

use common::{build_db_client, build_http_client, Charger, ChargerLambdaConfig, Email, HttpClient};

use crate::adapters::{AdapterError, build_email_client, CoordinatesDatabase, DbId, EmailClient};

mod adapters;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let lambda_config = Rc::new(
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

async fn flow<T: CoordinatesDatabase>(config: Rc<ChargerLambdaConfig>, db_client: Rc<T>, http_client: Rc<HttpClient>, email_client: Rc<EmailClient>) -> Result<Value, Error> {
    for item in db_client.get(config.get_table().0.as_str()).await? {
        let matches_given_id_for_this_item = |charger: &&Charger| &item.charger_id.0 == &charger.id;
        let chargers = http_client.get_chargers(item.ne_lat, item.ne_lon, item.sw_lat, item.sw_lon).await?;

        // we only want to send *one* email
        // point-free style
        let last_available_charger_if_any = chargers.iter()
            .filter(matches_given_id_for_this_item)
            .filter(has_available_connector)
            .last();

        if let Some(charger) = &last_available_charger_if_any {
            println!("Charger with id {} has available connectors!", charger.id);
            send_email_and_delete_item(&item.id, &item.email, config.clone(), db_client.clone(), email_client.clone()).await?;
        }
    };

    Ok(json!(
        { "message": "done" }
    ))
}

fn has_available_connector(charger: &&Charger) -> bool {
    charger.available_connectors > 0
}

async fn send_email_and_delete_item<T: CoordinatesDatabase>(id: &DbId, email: &Email, config: Rc<ChargerLambdaConfig>, db_client: Rc<T>, email_client: Rc<EmailClient>) -> Result<(), AdapterError> {
    email_client.send(email).await?;
    db_client.delete(config.get_table().0.as_str(), id).await
}
