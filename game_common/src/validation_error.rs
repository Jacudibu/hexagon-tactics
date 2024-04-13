#[derive(Debug)]
pub struct ValidationError {
    message: String,
}

impl ValidationError {
    pub fn new(message: &str) -> Self {
        ValidationError {
            message: message.into(),
        }
    }
}
