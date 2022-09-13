use std::rc::Rc;
use aws_sdk_ses::Client;
use aws_sdk_ses::model::{Body, Content, Destination, Message};
use common::{Email, SourceEmailAddress};
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

    pub async fn send(&self, to_email: &Email) -> Result<(), AdapterError> {
        let (message, source, destination) = build_email_message(self.source.as_str(), to_email);
        println!("Sending an email to {}", to_email.0);

        let _ = self.client.send_email()
            .set_source(source)
            .set_destination(destination)
            .message(message)
            .send()
            .await?;

        Ok(())
    }
}

// better message would be nice - body should contain some info about the actual connector
fn build_email_message(source: &str, destination: &Email) -> (Message, Option<String>, Option<Destination>) {
    let message = Message::builder()
        .subject(Content::builder()
            .data("Available connector!")
            .build())
        .body(Body::builder()
            .text(Content::builder()
                .data("There is a connector")
                .build())
            .build())
        .build();
    let source = Some(source.to_string());
    let destination = Some(Destination::builder().to_addresses(destination.0.to_string()).build());

    (message, source, destination)
}

pub async fn build_email_client(source: &SourceEmailAddress) -> Rc<EmailClient> {
    let config = aws_config::load_from_env().await;
    Rc::new(
        EmailClient::new(Client::new(&config), source.0.as_str())
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_build_an_email_message() {
        let (result_message, result_source, result_destination) = build_email_message("source@email.com", &Email("destination@email.com".to_string()));

        let subject = result_message.subject().unwrap().data().unwrap();
        let body = result_message.body().unwrap().text().unwrap().data().unwrap();
        let source = result_source.unwrap();
        let destinations = result_destination.unwrap();

        assert_eq!(subject, "Available connector!");
        assert_eq!(body, "There is a connector");
        assert_eq!(source, "source@email.com");
        assert_eq!(destinations.to_addresses().unwrap().get(0).unwrap(), "destination@email.com");
    }
}
