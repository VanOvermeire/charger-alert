use std::time::{SystemTime, UNIX_EPOCH};
use async_trait::async_trait;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::model::AttributeValue;
use common::{DB_ID_NAME, Lat, Lon};
use crate::adapters::AdapterError;

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
            .item(DB_ID_NAME, AttributeValue::S(id))
            .item(lon.get_name(), lon.into())
            .item(lat.get_name(), lat.into())
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
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
        .to_string()
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