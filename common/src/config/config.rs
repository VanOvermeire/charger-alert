use std::env;
use std::env::VarError;
use std::error::Error;
use std::fmt::{Display, Formatter};

const REGION_KEY: &'static str = "REGION";
const TABLE_KEY: &'static str = "TABLE";
const EMAIL: &'static str = "EMAIL";

#[derive(Debug)]
pub struct ConfigError {}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
         write!(f, "Internal server error")
    }
}

impl From<VarError> for ConfigError {
    fn from(_: VarError) -> Self {
        ConfigError {}
    }
}

impl Error for ConfigError {}

#[derive(Debug, Clone)]
pub struct Region(pub String); // instead provide a 'into'?
#[derive(Debug, Clone)]
pub struct Table(pub String);
#[derive(Debug, Clone)]
pub struct SourceEmailAddress(pub String);

#[derive(Debug, Clone)]
pub struct ChargerLambdaConfig {
    table: Table,
    region: Region,
    email: Option<SourceEmailAddress>
}

impl ChargerLambdaConfig {
    pub fn new() -> Result<ChargerLambdaConfig, ConfigError> {
        let region = env::var(REGION_KEY)?;
        let table = env::var(TABLE_KEY)?;
        let email = env::var(EMAIL).ok()
            .map(|e| SourceEmailAddress(e));

        Ok(ChargerLambdaConfig {
            table: Table(table),
            region: Region(region),
            email,
        })
    }

    pub fn get_table(&self) -> &Table {
        &self.table
    }

    pub fn get_region(&self) -> &Region {
        &self.region
    }

    pub fn get_email_address(&self) -> &SourceEmailAddress {
        &self.email.as_ref().expect("Email address to be available")
    }
}
