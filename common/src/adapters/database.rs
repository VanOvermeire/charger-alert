use aws_sdk_dynamodb::Client;

pub const DB_ID_NAME: &'static str = "id";

pub struct DynamoDB {
    client: Client,
}

impl DynamoDB {
    pub fn new(client: Client) -> Self {
        // maybe this could also handle the config part?
        DynamoDB {
            client,
        }
    }

    pub fn get_client_ref(&self) -> &Client {
        &self.client
    }
}
