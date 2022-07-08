#[derive(Debug)]
pub struct ErrorMessage(pub String);

impl ErrorMessage {
    pub fn new(message: String) -> Self {
        ErrorMessage(message)
    }
}