use std::collections::HashMap;
use common::{DbClient, NorthEastLatitude, NorthEastLongitude, SouthWestLatitude, SouthWestLongitude, Coordinate, DB_ID_NAME};
use async_trait::async_trait;
use aws_sdk_dynamodb::model::AttributeValue;
use crate::adapters::AdapterError;

pub struct DbId(String);

impl From<&DbId> for AttributeValue {
    fn from(id: &DbId) -> Self {
        AttributeValue::S(id.0.to_string()) // to string because cannot move
    }
}

pub struct ScanItem {
    pub id: DbId,
    pub ne_lat: NorthEastLatitude,
    pub ne_lon: NorthEastLongitude,
    pub sw_lat: SouthWestLatitude,
    pub sw_lon: SouthWestLongitude,
}

// why not put all this into the core (like by implementing try_into for coordinates?) mostly because I don't want to further contaminate that part of the code
// with stuff related to the db / attribute values / parsing / ...
impl TryFrom<&HashMap<String, AttributeValue>> for ScanItem {
    type Error = AdapterError;

    fn try_from(map: &HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        // we should not be able to put an item in our db without a string id. so looser error handling for the id
        let id = map.get(DB_ID_NAME).expect("Database item to have an id").as_s().expect("Database id to be a string").to_string();
        let ne_lat = from_map_to_coordinate(map)?;
        let ne_lon = from_map_to_coordinate(map)?;
        let sw_lat = from_map_to_coordinate(map)?;
        let sw_lon = from_map_to_coordinate(map)?;

        Ok(ScanItem {
            id: DbId(id),
            ne_lat,
            ne_lon,
            sw_lat,
            sw_lon,
        })
    }
}

// originally passed in the name, but that left room for errors, replaced the constructor fn that I passed in with ::new, which I added
// Rust elegantly infers the correct types. writing even shorter code by trying to map over this function 4 times and letting Rust do the rest
// did not work out
fn from_map_to_coordinate<C: Coordinate>(map: &HashMap<String, AttributeValue>) -> Result<C, AdapterError> {
    map.get(C::get_type_name()).ok_or_else(|| AdapterError::ParseError)
        .and_then(|v| v.as_n().map_err(|_| AdapterError::ParseError))
        .and_then(|v| v.parse::<f32>().map(C::new).map_err(|_| AdapterError::ParseError))

    // alternative with less error handling thanks to pattern matching, but a bit harder to read the mappings //
    // match map.get(C::get_type_name()).map(|v| v.as_n().map(|v| v.parse::<f32>().map(C::new))) {
    //     Some(Ok(Ok(res))) => Ok(res),
    //     _ => Err(AdapterError::ParseError)
    // }
}

#[async_trait]
pub trait CoordinatesDatabase {
    async fn get(&self, table: &str) -> Result<Vec<ScanItem>, AdapterError>;
    async fn delete(&self, table: &str, id: &DbId) -> Result<(), AdapterError>;
}

#[async_trait]
impl CoordinatesDatabase for DbClient {
    async fn get(&self, table: &str) -> Result<Vec<ScanItem>, AdapterError> {
        let scan_result = &self.get_client_ref().scan()
            .table_name(table)
            .send().await?;
        // again, neat. automatic transform of what was a Vec of Results into a Result of Vec!
        scan_result.items.as_ref().map(|vec|
            vec.into_iter()
                .map(|v| v.try_into())
                .collect()
        ).unwrap_or(Ok(vec![]))
    }

    async fn delete(&self, table: &str, id: &DbId) -> Result<(), AdapterError> {
        let _ = &self.get_client_ref().delete_item()
            .table_name(table)
            .key(DB_ID_NAME, id.into())
            .send().await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use aws_sdk_dynamodb::model::AttributeValue;
    use crate::adapters::{AdapterError, ScanItem};

    #[test]
    fn should_change_a_hashmap_into_a_scan_item() {
        let ref input = HashMap::from([
            ("id".to_string(), AttributeValue::S("12345".to_string())),
            ("nelat".to_string(), AttributeValue::N("55".to_string())),
            ("nelon".to_string(), AttributeValue::N("22.2".to_string())),
            ("swlon".to_string(), AttributeValue::N("17.1".to_string())),
            ("swlat".to_string(), AttributeValue::N("1".to_string())),
        ]);

        let result: ScanItem = input.try_into().expect("Try into to succeed");

        assert_eq!(result.id, "12345");
        assert_eq!(result.ne_lat.0, 55.0);
        assert_eq!(result.ne_lon.0, 22.2);
        assert_eq!(result.sw_lat.0, 1.0);
        assert_eq!(result.sw_lon.0, 17.1);
    }

    #[test]
    fn should_return_an_adapter_error_when_a_value_is_missing() {
        let ref input = HashMap::from([
            ("id".to_string(), AttributeValue::S("12345".to_string())),
            ("something else".to_string(), AttributeValue::N("55".to_string())),
            ("nelon".to_string(), AttributeValue::N("22.2".to_string())),
            ("swlon".to_string(), AttributeValue::N("17.1".to_string())),
            ("swlat".to_string(), AttributeValue::N("1".to_string())),
        ]);

        let result: Result<ScanItem, AdapterError> = input.try_into();

        assert!(result.is_err());
    }

    #[test]
    fn should_return_an_adapter_error_when_value_is_attribute_value_string() {
        let ref input = HashMap::from([
            ("id".to_string(), AttributeValue::S("12345".to_string())),
            ("nelat".to_string(), AttributeValue::N("55".to_string())),
            ("nelon".to_string(), AttributeValue::S("22.2".to_string())),
            ("swlon".to_string(), AttributeValue::N("17.1".to_string())),
            ("swlat".to_string(), AttributeValue::N("1".to_string())),
        ]);

        let result: Result<ScanItem, AdapterError> = input.try_into();

        assert!(result.is_err());
    }

    #[test]
    fn should_return_an_adapter_error_when_value_is_a_string() {
        let ref input = HashMap::from([
            ("id".to_string(), AttributeValue::S("12345".to_string())),
            ("nelat".to_string(), AttributeValue::N("55".to_string())),
            ("nelon".to_string(), AttributeValue::N("22.2".to_string())),
            ("swlon".to_string(), AttributeValue::N("fake".to_string())),
            ("swlat".to_string(), AttributeValue::N("1".to_string())),
        ]);

        let result: Result<ScanItem, AdapterError> = input.try_into();

        assert!(result.is_err());
    }
}
