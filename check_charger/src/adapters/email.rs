use std::sync::Arc;
use aws_sdk_ses::Client;
use aws_sdk_ses::model::{Body, Content, Destination, Message};
use common::SourceEmailAddress;
use crate::adapters::AdapterError;

pub struct EmailClient {
    source: String,
    client: Client
}

impl EmailClient {
    pub fn new(client: Client, source: &str) -> Self {
        EmailClient {
            client,
            source: source.to_string(),
        }
    }

    pub async fn send(&self, destination: &str) -> Result<(), AdapterError> {
        // TODO this can be tested - pull it out
        let message = Message::builder()
            .subject(Content::builder().data("Available connector").build())
            .body(Body::builder().text(Content::builder().data("There is a connector").build()).build())
            .build();
        let source = Some(self.source.to_string());
        let destination = Some(Destination::builder().to_addresses(destination.to_string()).build());

        let _ = self.client.send_email()
            .set_source(source)
            .set_destination(destination)
            .message(message)
            .send()
            .await?;

        Ok(())
    }
}

pub async fn build_email_client(source: &SourceEmailAddress) -> Arc<EmailClient> {
    let config = aws_config::load_from_env().await;
    Arc::new(
        EmailClient::new(Client::new(&config), source.0.as_str())
    )
}
