use std::str::Utf8Error;
use tokio::sync::mpsc::error::SendError;

use axum::{extract::ws::Message, response::IntoResponse};
use thiserror::Error;

pub const AUTH_SERVER: &str = "http://localhost:3345";

mod ack;
pub use ack::*;

mod send_message;
pub use send_message::*;

mod connect_with;
pub use connect_with::*;
mod connect_response;
pub use connect_response::*;

mod show_invites;
pub use show_invites::*;

mod show_metadata;
pub use show_metadata::*;

mod show_chats;
pub use show_chats::*;

#[derive(Error, Debug)]
pub enum HandleError {
    #[error("Unknown instruction: {0}")]
    UnknownInstruct(i32),
    #[error("Failed to handle instruction: {0}")]
    HandleError(String),
    #[error("Failed to send: {0}")]
    SendError(#[from] SendError<Message>),
    #[error("Failed to register message channel for user: {0}")]
    ChannelError(String),
    #[error("No message channel found for user")]
    NoChannelFound,

    #[error("No user found: {0}")]
    NoUserFound(String),
    #[error("No chat found: {0}")]
    NoChatFound(String),
    #[error("No message found: {0}")]
    NoMessageFound(String),
    #[error("No invitation found: {0}")]
    NoInvitationFound(String),

    #[error("System time error: {0}")]
    SystemTimeError(#[from] std::time::SystemTimeError),
    #[error("Invalid credentials: {0}")]
    InvalidCredentials(String),
    #[error("Token expired")]
    TokenExpired,
    #[error("Hashing failed: {0}")]
    HashError(String),

    #[error("Axum error")]
    AxumError(#[from] axum::Error),

    // rpc errors
    #[error("RPC error: {0}")]
    RpcError(#[from] reqwest::Error),

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

impl IntoResponse for HandleError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            // HandleError::NoUserFound(_) => axum::http::StatusCode::UNAUTHORIZED,
            HandleError::NoChatFound(_) => axum::http::StatusCode::NOT_FOUND,
            HandleError::NoMessageFound(_) => axum::http::StatusCode::NOT_FOUND,
            HandleError::NoInvitationFound(_) => axum::http::StatusCode::NOT_FOUND,
            _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, self.to_string()).into_response()
    }
}
