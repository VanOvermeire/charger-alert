use async_trait::async_trait;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::model::AttributeValue;
use chrono::Utc;
use crate::adapters::{AdapterError, Lat, Lon};

#[async_trait]
pub trait Database {
    async fn add_lat_and_lon(&self, table: &str, lat: &Lat, lon: &Lon) -> Result<(), AdapterError>;
}

pub struct DynamoDB {
    client: Client,
}

impl DynamoDB {
    pub fn new(client: Client) -> DynamoDB {
        DynamoDB {
            client,
        }
    }
}

#[async_trait]
impl Database for DynamoDB {
    async fn add_lat_and_lon(&self, table: &str, lat: &Lat, lon: &Lon) -> Result<(), AdapterError> {
        let id = format!("id-{}", Utc::now());

        match &self.client.put_item()
            .table_name(table)
            .item("id", AttributeValue::S(id))
            .item("longitude", AttributeValue::N(lon.0.to_string()))
            .item("latitude", AttributeValue::N(lat.0.to_string()))
            .send()
            .await {
            Ok(_) => Ok(()),
            Err(e) => Err(AdapterError::DatabaseError),
        }
    }
}
