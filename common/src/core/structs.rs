use serde::{Deserialize};
use aws_sdk_dynamodb::model::AttributeValue;

#[derive(Deserialize, Debug)]
pub struct NorthEastLatitude(pub f32);
#[derive(Deserialize, Debug)]
pub struct NorthEastLongitude(pub f32);
#[derive(Deserialize, Debug)]
pub struct SouthWestLatitude(pub f32);
#[derive(Deserialize, Debug)]
pub struct SouthWestLongitude(pub f32);

macro_rules! generate_name_for_coordinate {
    ($coordinate_type:ty,$name:literal) => {
        impl $coordinate_type {
            pub fn get_name(&self) -> &'static str {
                $name
            }
        }
    };
}

macro_rules! generate_from_for_coordinate {
    ($coordinate_type:ty) => {
        impl From<&$coordinate_type> for AttributeValue {
            fn from(l: &$coordinate_type) -> Self {
                AttributeValue::N(l.0.to_string())
            }
        }
    };
}

macro_rules! generate_methods_for_coordinate {
        ($coordinate_type:ty,$name:literal) => {
            generate_name_for_coordinate!($coordinate_type, $name);
            generate_from_for_coordinate!($coordinate_type);
    };
}

// can't take constants, so passed in as str
generate_methods_for_coordinate!(NorthEastLatitude, "nelat");
generate_methods_for_coordinate!(NorthEastLongitude, "nelon");
generate_methods_for_coordinate!(SouthWestLatitude, "swlat");
generate_methods_for_coordinate!(SouthWestLongitude, "swlon");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_a_valid_into_attribute_value() {
        let value = &SouthWestLongitude(55.1);

        let result: AttributeValue = value.into();

        match result {
            AttributeValue::N(val) => assert_eq!(val, "55.1"),
            _ => panic!("Unexpected attribute value (not a number)")
        }
    }

    #[test]
    fn should_generate_a_valid_get_name_method() {
        let value = &SouthWestLatitude(55.1);

        assert_eq!(value.get_name(), "swlat");
    }
}
