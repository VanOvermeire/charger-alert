use std::collections::HashMap;
use common::{DynamoDB, NorthEastLatitude, NorthEastLongitude, SouthWestLatitude, SouthWestLongitude, Coordinate};
use async_trait::async_trait;
use aws_sdk_dynamodb::model::AttributeValue;
use serde_json::to_string;
use crate::adapters::AdapterError;

#[async_trait]
trait CoordinatesDatabase {
    async fn get(&self);
}

pub struct ScanItem {
    ne_lat: NorthEastLatitude,
    ne_lon: NorthEastLongitude,
    sw_lat: SouthWestLatitude,
    sw_lon: SouthWestLongitude,
}

// why not put all this into the core (like by implementing try_into for coordinates?) mostly because I don't want to further contaminate that part of the code
// with stuff related to the db / attribute values / parsing / ...
impl TryFrom<HashMap<String, AttributeValue>> for ScanItem {
    type Error = AdapterError;

    fn try_from(map: HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        let ne_lat = from_map_to_coordinate(&map,|v| NorthEastLatitude(v))?;
        let ne_lon = from_map_to_coordinate(&map,|v| NorthEastLongitude(v))?;
        let sw_lat = from_map_to_coordinate(&map, |v| SouthWestLatitude(v))?;
        let sw_lon = from_map_to_coordinate(&map, |v| SouthWestLongitude(v))?;

        Ok(ScanItem {
            ne_lat,
            ne_lon,
            sw_lat,
            sw_lon,
        })
    }
}

// originally passed in the name, but that left room for errors
fn from_map_to_coordinate<C: Coordinate>(map: &HashMap<String, AttributeValue>, constructor: fn(f32) -> C) -> Result<C, AdapterError> {
    map.get(C::get_type_name())
        .ok_or_else(|| AdapterError::ParseError)
        .and_then(|v| v.as_n().map_err(|_| AdapterError::ParseError))
        .and_then(|v| v.parse::<f32>().map(constructor).map_err(|_| AdapterError::ParseError))

}

#[async_trait]
impl CoordinatesDatabase for DynamoDB {
    async fn get(&self) {
        // , table: &str

        // let result = &self.get_client_ref()
        //     .scan()
        //     .table_name(table)
        //     .send().await?;
        // let t = result.items;


    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use aws_sdk_dynamodb::model::AttributeValue;
    use crate::adapters::ScanItem;

    #[test]
    fn should_change_a_hashmap_into_a_scan_item() {
        let input = HashMap::from([
            ("nelat".to_string(), AttributeValue::N("55".to_string())),
            ("nelon".to_string(), AttributeValue::N("22.2".to_string())),
            ("swlon".to_string(), AttributeValue::N("17.1".to_string())),
            ("swlat".to_string(), AttributeValue::N("1".to_string())),
        ]);

        let result: ScanItem = input.try_into().expect("Try into to succeed");

        assert_eq!(result.ne_lat.0, 55.0);
        assert_eq!(result.ne_lon.0, 22.2);
        assert_eq!(result.sw_lat.0, 1.0);
        assert_eq!(result.sw_lon.0, 17.1);
    }

}
