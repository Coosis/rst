use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Infallible error")]
    None,
    #[error("An error occurred: {0}")]
    Unknown(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Connection error: {0}")]
    ConnectionError(String),
    #[error("Not connected")]
    NotConnected,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serde error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
}
