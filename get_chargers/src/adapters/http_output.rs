use serde::{Serialize};
use common::Charger;

#[derive(Serialize, Debug)]
pub struct GetChargerOutput {
    charger_id: i32,
    latitude: f32,
    longitude: f32,
    available_connectors: i8,
}

impl From<Charger> for GetChargerOutput {
    fn from(c: Charger) -> Self {
        GetChargerOutput {
            charger_id: c.id,
            latitude: c.lat,
            longitude: c.lng,
            available_connectors: c.available_connectors,
        }
    }
}
