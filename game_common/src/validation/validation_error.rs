use crate::network_events::server_to_client::{ErrorWhenProcessingMessage, ServerToClientMessage};

#[derive(Debug)]
pub struct ValidationError {
    message: String,
}

impl ValidationError {
    #[must_use]
    pub fn new<T: Into<String>>(message: T) -> Self {
        ValidationError {
            message: message.into(),
        }
    }
}

impl From<ValidationError> for ServerToClientMessage {
    fn from(value: ValidationError) -> Self {
        ServerToClientMessage::ErrorWhenProcessingMessage(ErrorWhenProcessingMessage {
            message: value.message,
        })
    }
}
