use std::time::{SystemTime, UNIX_EPOCH};
use async_trait::async_trait;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::model::AttributeValue;
use crate::adapters::{AdapterError, Lat, Lon};

#[async_trait]
pub trait Database {
    async fn add_lat_and_lon(&self, table: &str, lat: &Lat, lon: &Lon) -> Result<(), AdapterError>;
}

pub struct DynamoDB {
    client: Client,
}

impl DynamoDB {
    pub fn new(client: Client) -> Self {
        DynamoDB {
            client,
        }
    }
}

#[async_trait]
impl Database for DynamoDB {
    async fn add_lat_and_lon(&self, table: &str, lat: &Lat, lon: &Lon) -> Result<(), AdapterError> {
        let id = generate_id();

        match &self.client.put_item()
            .table_name(table)
            .item("id", AttributeValue::S(id))
            .item("longitude", AttributeValue::N(lon.0.to_string()))
            .item("latitude", AttributeValue::N(lat.0.to_string()))
            .send()
            .await {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Error from database: {:?}", e);
                Err(AdapterError::DatabaseError)
            },
        }
    }
}

fn generate_id() -> String {
    let since_the_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis().to_string()
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use super::*;

    #[test]
    fn should_generate_an_epoch_millis_id() {
        let re = Regex::new(r"^\d{13}$").unwrap();

        let result = generate_id();

        assert!(re.is_match(result.as_str())); // this will fail around 2282 AD
    }
}