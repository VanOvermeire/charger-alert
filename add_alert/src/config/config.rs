use std::env;
use crate::adapters::AdapterError;

const REGION_KEY: &'static str = "REGION";
const TABLE_KEY: &'static str = "TABLE";

#[derive(Debug, Clone)]
pub struct Region(pub String); // instead provide a 'into'?
#[derive(Debug, Clone)]
pub struct Table(pub String);

#[derive(Debug, Clone)]
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
