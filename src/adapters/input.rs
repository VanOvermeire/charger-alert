use std::env;
use crate::adapters::AdapterError;
use lambda_http::{Request, RequestExt};
use serde::{Deserialize};

const REGION_KEY: &'static str = "REGION";
const TABLE_KEY: &'static str = "TABLE";

pub struct Region(String);
pub struct Table(String);

pub struct Config {
    table: Table,
    region: Region,
}

impl Config {
    pub fn new() -> Result<Config, AdapterError> {
        let region = env::var(REGION_KEY)?;
        let table = env::var(TABLE_KEY)?;

        Ok(Config {
            table: Table(table),
            region: Region(region),
        })
    }

    pub fn get_table(&self) -> &Table {
        &self.table
    }

    pub fn get_region(&self) -> &Region {
        &self.region
    }
}

#[derive(Deserialize)]
pub struct Lat(f32);
#[derive(Deserialize)]
pub struct Lon(f32);

#[derive(Deserialize)]
pub struct ChargerRequest {
    lat: Lat,
    lon: Lon,
}

impl ChargerRequest {
    pub fn get_lat_and_long(&self) -> (&Lat, &Lon) {
        (&self.lat, &self.lon)
    }
}

impl TryInto<ChargerRequest> for Request {
    type Error = AdapterError;

    fn try_into(self) -> Result<ChargerRequest, Self::Error> {
        self.payload()?.ok_or(AdapterError::InputError)
    }
}
