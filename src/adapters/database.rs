use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
// use aws_sdk_dynamodb::{Client, SdkError};
// use aws_sdk_dynamodb::error::PutItemError;
// use aws_sdk_dynamodb::model::AttributeValue;
use async_trait::async_trait;
use crate::adapters::AdapterError;

// TODO tests

#[async_trait]
pub trait Database {
    async fn add<T: ToString + Send>(table: &str, items: HashMap<&str, T>) -> Result<(), AdapterError> {
        Ok(())
    }
}

// use this for the implementation
// async fn add(client: Client, table: &str, longitude: f32, latitude: f32) -> Result<(), SdkError<PutItemError>> {
//     let key = AttributeValue::S("some-key".to_string()); // TODO
//     let longitude_as_attribute_value = AttributeValue::N(longitude.to_string());
//     let latitude_as_attribute_value = AttributeValue::N(latitude.to_string());
//
//     match client.put_item().table_name(table)
//         .item("id", key)// TODO can we use type to make sure that an id is present?
//         .item("longitude", longitude_as_attribute_value)
//         .item("latitude", latitude_as_attribute_value)
//         .send()
//         .await {
//         Ok(_) => Ok(()),
//         Err(e) => Err(e),
//     }
// }
