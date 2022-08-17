use common::DynamoDB;
use async_trait::async_trait;

#[async_trait]
trait CoordinatesDatabase {
    async fn get(&self);
}

#[async_trait]
impl CoordinatesDatabase for DynamoDB {
    async fn get(&self) {
        let result = &self.get_client_ref().scan().send().await;
    }
}