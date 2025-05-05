use argon2::Argon2;
use tracing::{debug, info};
use axum::{self, routing::{any, get}, Router};
use std::sync::Arc;

mod error;
pub use error::*;
mod db;
pub use db::*;
mod handlers;
pub use handlers::*;

const SECRET: &[u8] = "secret".as_bytes();

type Result<T> = std::result::Result<T, AuthError>;

pub struct AppState {
    db_client: mongodb::Client,
    // kv_client: redis::Client,
    argon: argon2::Argon2<'static>,
    // authed_tx: std::collections::HashMap<Uuid, tokio::sync::mpsc::UnboundedSender<Message>>,
}

type SharedState = std::sync::Arc<AppState>;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    debug!("Starting server");
    let client = mongodb::Client::with_uri_str(crate::MONGODB_URI).await.unwrap();
    let state: SharedState = Arc::new(AppState {
        db_client: client,
        argon: Argon2::default(),
    });
    let app = Router::new()
        .route("/auth", any(auth))
        .route("/register", any(register))
        .route("/get_uid", any(get_uid))
        .route("/get_metadata", any(get_credentials))
        .with_state(state.clone());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3345").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

