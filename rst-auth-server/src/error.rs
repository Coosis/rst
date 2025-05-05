use axum::response::IntoResponse;
use thiserror::Error;
use serde::Deserialize;
use std::str::Utf8Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Unknown instruction: {0}")]
    UnknownInstruct(i32),
    #[error("Failed to handle instruction: {0}")]
    HandleError(String),
    #[error("Failed to send: {0}")]
    SendError(String),

    #[error("No user found: {0}")]
    NoUserFound(String),
    // #[error("No chat found: {0}")]
    // NoChatFound(String),
    // #[error("No message found: {0}")]
    // NoMessageFound(String),
    // #[error("No invitation found: {0}")]
    // NoInvitationFound(String),

    #[error("System time error: {0}")]
    SystemTimeError(#[from] std::time::SystemTimeError),
    #[error("Invalid credentials: {0}")]
    InvalidCredentials(String),
    #[error("Hashing failed: {0}")]
    HashError(String),

    #[error("Axum error")]
    AxumError(#[from] axum::Error),

    // parsing / serde && serde_json errors
    #[error("Invalid utf-8 string: {0}")]
    Utf8Error(#[from] Utf8Error),
    #[error("Serialization error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    // DB related errors
    #[error("Failed to connect to database: {0}")]
    DbConnectionError(String),
    #[error("Database error: {0}")]
    DbError(#[from] mongodb::error::Error),
    #[error("bson error: {0}")]
    BsonDeError(#[from] bson::de::Error),
    #[error("bson serialization error: {0}")]
    BsonSerError(#[from] bson::ser::Error),

    // In-mem cache related errors
    // #[error("Failed to connect to in-mem cache: {0}")]
    // CacheConnectionError(String),
    // #[error("Failed to multiplex connection")]
    // MultiplexError,
    // #[error("Cache error: {0}")]
    // CacheError(#[from] redis::RedisError),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            AuthError::NoUserFound(_) => axum::http::StatusCode::NOT_FOUND,
            AuthError::InvalidCredentials(_) => axum::http::StatusCode::UNAUTHORIZED,
            _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, self.to_string()).into_response()
    }
}
