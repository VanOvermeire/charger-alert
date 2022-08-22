use std::rc::Rc;
use crate::adapters::AdapterError;
use reqwest::{Client};
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use common::{NorthEastLatitude, NorthEastLongitude, SouthWestLatitude, SouthWestLongitude};

// for external use, unlike the below that map what we receive from the endpoint
#[derive(Debug)]
pub struct Charger {
    pub id: i32,
    pub lat: f32,
    pub lng: f32,
    pub available_connectors: i8,
}

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

const BASE_URL: &str = "https://nl.chargemap.com/json/charging/pools/get_from_areas";

pub struct HttpClient {
    client: Client,
}

impl Default for HttpClient {
    fn default() -> Self {
        HttpClient::new(Client::new())
    }
}

impl HttpClient {
    fn new(client: Client) -> Self {
        HttpClient {
            client,
        }
    }

    pub async fn get_chargers(&self, ne_lat: NorthEastLatitude, ne_lon: NorthEastLongitude, sw_lat: SouthWestLatitude, sw_lon: SouthWestLongitude) -> Result<Vec<Charger>, AdapterError> {
        self.get(ne_lat, ne_lon, sw_lat, sw_lon).await
            .map(charger_info_to_chargers)
    }

    // internally we do a post, but it doesn't actually change anything. so get seems like a fitting name
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

fn charger_info_to_chargers(info: ChargerInfo) -> Vec<Charger> {
    info.items.into_iter().map(|i| {
        Charger {
            id: i.pool.id,
            lat: i.lat,
            lng: i.lng,
            available_connectors: count(&i.pool.charging_connectors, |c| c.available_count)
        }
    }).collect()
}

fn count(connectors: &Vec<Connectors>, field_supplier: fn(&Connectors) -> i8) -> i8 {
    connectors.iter().map(field_supplier).sum()
}

pub async fn build_http_client() -> Rc<HttpClient> {
    Rc::new(
        HttpClient::default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_change_charging_info_to_chargers() {
        let first = Item {
            lat: 10.12,
            lng: 5.1,
            pool: Pool {
                id: 1,
                name: "first".to_string(),
                charging_connectors: vec![
                    Connectors {
                        count: 2,
                        available_count: 1
                    },
                    Connectors {
                        count: 3,
                        available_count: 1
                    }
                ]
            }
        };
        let second = Item {
            lat: 22.4,
            lng: 57.8,
            pool: Pool {
                id: 2,
                name: "second".to_string(),
                charging_connectors: vec![]
            }
        };

        let info = ChargerInfo {
            count: 2,
            items: vec![first, second]
        };

        let result = charger_info_to_chargers(info);

        assert_eq!(result.len(), 2);

        let first_result = result.get(0).unwrap();

        assert_eq!(first_result.id, 1);
        assert_eq!(first_result.lat, 10.12);
        assert_eq!(first_result.lng, 5.1);
        assert_eq!(first_result.available_connectors, 2);

        let second_result = result.get(1).unwrap();

        assert_eq!(second_result.id, 2);
        assert_eq!(second_result.lat, 22.4);
        assert_eq!(second_result.lng, 57.8);
        assert_eq!(second_result.available_connectors, 0);
    }
}