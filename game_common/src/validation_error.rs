#[derive(Debug)]
pub struct ValidationError {
    message: String,
}

impl ValidationError {
    #[must_use]
    pub fn new(message: &str) -> Self {
        ValidationError {
            message: message.into(),
        }
    }
}
