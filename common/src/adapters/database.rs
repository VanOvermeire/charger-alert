use aws_sdk_dynamodb::Client;

pub const DB_ID_NAME: &'static str = "id";

pub struct DynamoDB {
    client: Client,
}

impl DynamoDB {
    pub fn new(client: Client) -> Self {
        DynamoDB {
            client,
        }
    }

    pub fn get_client_ref(&self) -> &Client {
        &self.client
    }
}
