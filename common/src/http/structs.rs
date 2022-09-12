use serde::{Deserialize, Serialize};

// for external use
#[derive(Debug)]
pub struct Charger {
    pub id: i32,
    pub lat: f32,
    pub lng: f32,
    pub available_connectors: i8,
}

// for internal use
#[derive(Debug, Serialize, Deserialize)]
pub struct ChargerInfo {
    pub count: u32,
    pub items: Vec<Item>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub lat: f32,
    pub lng: f32,
    pub pool: Pool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pool {
    pub id: i32,
    pub name: String,
    pub charging_connectors: Vec<Connectors>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Connectors {
    pub count: i8,
    pub available_count: i8,
}

