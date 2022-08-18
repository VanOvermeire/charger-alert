use reqwest::Client;
use serde::{Deserialize, Serialize};
use common::{NorthEastLatitude, NorthEastLongitude, SouthWestLatitude, SouthWestLongitude};

#[derive(Debug, Serialize, Deserialize)]
struct Results {
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

    fn get(&self) {


        // let client = reqwest::blocking::Client::new();
        // let data = "departure%5Blat%5D=50.844837&departure%5Blng%5D=4.39695&NELat=50.84691587&NELng=4.4037956&SWLat=50.84283428&SWLng=4.39634442";
        // let res = client.post("https://nl.chargemap.com/json/charging/pools/get_from_areas")
        // .header("content-type", "application/x-www-form-urlencoded; charset=UTF-8")
        // .body(data)
        // .send()?
        // .json::<Results>()?
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        HttpClient {
            client: Client::new(),
        }
    }
}


