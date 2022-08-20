use crate::adapters::AdapterError;
use std::fmt::format;
use reqwest::Client;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use common::{NorthEastLatitude, NorthEastLongitude, SouthWestLatitude, SouthWestLongitude};

#[derive(Debug, Serialize, Deserialize)]
struct ChargerInfo {
    count: u32,
    items: Vec<Item>
}

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    lat: f32,
    lng: f32,
    pool: Pool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Pool {
    id: i32,
    name: String,
    charging_connectors: Vec<Connectors>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Connectors {
    count: i8,
    available_count: i8,
}

pub struct GetInput {
    ne_lat: NorthEastLatitude,
    ne_lon: NorthEastLongitude,
    sw_lat: SouthWestLatitude,
    sw_lon: SouthWestLongitude,
}

const BASE_URL: &str = "https://nl.chargemap.com/json/charging/pools/get_from_areas";

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    fn new(client: Client) -> Self {
        HttpClient {
            client,
        }
    }

    async fn get(&self, ne_lat: NorthEastLatitude, ne_lon: NorthEastLongitude, sw_lat: SouthWestLatitude, sw_lon: SouthWestLongitude) -> Result<ChargerInfo, AdapterError> {
        let body = format!("NELat={}&NELng={}&SWLat={}&SWLng={}", ne_lat.0, ne_lon.0, sw_lat.0, sw_lon.0);

        Ok(self.client.post(BASE_URL)
            .body(body)
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded; charset=UTF-8")
            .send()
            .await?
            .json::<ChargerInfo>()
            .await?)
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        HttpClient {
            client: Client::new(),
        }
    }
}
