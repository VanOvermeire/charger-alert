use std::rc::Rc;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, Region};
use crate::config;

pub const DB_ID_NAME: &'static str = "id";
pub const DB_EMAIL_NAME: &'static str = "email";

pub struct DbClient {
    client: Client,
}

impl DbClient {
    pub fn new(client: Client) -> Self {
        DbClient {
            client,
        }
    }

    pub fn get_client_ref(&self) -> &Client {
        &self.client
    }
}

pub async fn build_db_client(region: &config::Region) -> Rc<DbClient> {
    let region_provider = RegionProviderChain::first_try(Region::new(region.0.clone())).or_default_provider();
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    Rc::new(DbClient::new(Client::new(&shared_config)))
}
