use std::time::{SystemTime, UNIX_EPOCH};
use async_trait::async_trait;
use aws_sdk_dynamodb::model::AttributeValue;
use common::{DB_ID_NAME, DbClient, NorthEastLatitude, NorthEastLongitude, SouthWestLatitude, SouthWestLongitude, Coordinate, Email, DB_EMAIL_NAME};
use crate::adapters::AdapterError;

#[async_trait]
pub trait CoordinatesDb {
    async fn add(&self,
                 table: &str, email: Email,
                 ne_lat: NorthEastLatitude, ne_lon: NorthEastLongitude,
                 sw_lat: SouthWestLatitude, sw_lon: SouthWestLongitude) -> Result<(), AdapterError>;
}

#[async_trait]
impl CoordinatesDb for DbClient {
    async fn add(&self,
                 table: &str, email: Email,
                 ne_lat: NorthEastLatitude, ne_lon: NorthEastLongitude,
                 sw_lat: SouthWestLatitude, sw_lon: SouthWestLongitude) -> Result<(), AdapterError> {
        let id = generate_id();

        match &self.get_client_ref().put_item()
            .table_name(table)
            .item(DB_ID_NAME, AttributeValue::S(id))
            .item(DB_EMAIL_NAME, AttributeValue::S(email.0))
            .item(ne_lon.get_name(), ne_lon.into())
            .item(ne_lat.get_name(), ne_lat.into())
            .item(sw_lat.get_name(), sw_lat.into())
            .item(sw_lon.get_name(), sw_lon.into())
            .send()
            .await {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Error from database: {:?}", e);
                Err(AdapterError::DatabaseError)
            }
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