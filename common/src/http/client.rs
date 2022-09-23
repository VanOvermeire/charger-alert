use std::rc::Rc;
use reqwest::{Client};
use reqwest::header::CONTENT_TYPE;
use crate::{NorthEastLatitude, NorthEastLongitude, SouthWestLatitude, SouthWestLongitude};
use crate::http::errors::HttpError;
use crate::http::structs::{Connectors, ChargerInfo, Charger};

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

    pub async fn get_chargers(&self, ne_lat: NorthEastLatitude, ne_lon: NorthEastLongitude, sw_lat: SouthWestLatitude, sw_lon: SouthWestLongitude) -> Result<Vec<Charger>, HttpError> {
        self.get(ne_lat, ne_lon, sw_lat, sw_lon).await
            .map(charger_info_to_chargers)
    }

    // internally we do a post, but it doesn't actually change anything. so 'get' seems like a fitting name
    async fn get(&self, ne_lat: NorthEastLatitude, ne_lon: NorthEastLongitude, sw_lat: SouthWestLatitude, sw_lon: SouthWestLongitude) -> Result<ChargerInfo, HttpError> {
        Ok(self.client.post(BASE_URL)
            .body(build_get_body(ne_lat, ne_lon, sw_lat, sw_lon))
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded; charset=UTF-8")
            .send()
            .await?
            .json::<ChargerInfo>()
            .await?)
    }
}

pub async fn build_http_client() -> Rc<HttpClient> {
    Rc::new(
        HttpClient::default(),
    )
}

fn build_get_body(ne_lat: NorthEastLatitude, ne_lon: NorthEastLongitude, sw_lat: SouthWestLatitude, sw_lon: SouthWestLongitude) -> String {
    format!("NELat={}&NELng={}&SWLat={}&SWLng={}", ne_lat.0, ne_lon.0, sw_lat.0, sw_lon.0)
}

fn charger_info_to_chargers(info: ChargerInfo) -> Vec<Charger> {
    info.items.into_iter().map(|i| {
        Charger {
            id: i.pool.id,
            lat: i.lat,
            lng: i.lng,
            available_connectors: count(&i.pool.charging_connectors, |c| c.available_count),
        }
    }).collect()
}

fn count(connectors: &Vec<Connectors>, field_supplier: fn(&Connectors) -> i8) -> i8 {
    connectors.iter().map(field_supplier).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::structs::{Item, Pool};

    #[test]
    fn should_build_correct_body_for_get() {
        let ne_lat = NorthEastLatitude(1.1);
        let ne_lon = NorthEastLongitude(2.0);
        let sw_lat = SouthWestLatitude(3.2);
        let sw_lon = SouthWestLongitude(4.3);

        let result = build_get_body(ne_lat, ne_lon, sw_lat, sw_lon);

        assert_eq!(result, "NELat=1.1&NELng=2&SWLat=3.2&SWLng=4.3".to_string());
    }

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
                        available_count: 1,
                    },
                    Connectors {
                        count: 3,
                        available_count: 1,
                    },
                ],
            },
        };
        let second = Item {
            lat: 22.4,
            lng: 57.8,
            pool: Pool {
                id: 2,
                name: "second".to_string(),
                charging_connectors: vec![],
            },
        };

        let info = ChargerInfo {
            count: 2,
            items: vec![first, second],
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