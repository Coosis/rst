use std::collections::HashMap;

use argon2::Argon2;
use axum::extract::ws::Message;
use lib::user::UserCredential;
use mongodb::Client;
use redis::AsyncCommands;
use lib::Uuid;
use tracing::debug;

use crate::handlers::HandleError;
use crate::MsgChan;

type Result<T> = std::result::Result<T, HandleError>;

pub struct AppState {
    /// session id -> message channel
    // pub tx: HashMap<Uuid, MsgChan>,
    /// Used for message routing,
    /// user id -> message channel
    pub authed_tx: HashMap<Uuid, MsgChan>,
    pub db_client: mongodb::Client,
    // pub kv_client: redis::aio::MultiplexedConnection,
    pub argon: Argon2<'static>,
    pub client: reqwest::Client,
}

impl AppState {
    /// Create a new AppState
    ///
    /// # Arguments
    /// * `uri` - A string slice that holds the URI for the MongoDB connection
    /// * `kv_uri` - A string slice that holds the URI for the Redis connection
    pub async fn new(
        uri: &str,
        kv_uri: &str
    ) -> Result<Self> {
        // DB connection
        let db_client = match Client::with_uri_str(uri).await {
            Ok(c) => c,
            Err(_) => return Err(
                HandleError::DbConnectionError(
                    "Failed to connect to MongoDB".to_string()
                )
            )
        };

        // Cache connection
        // let client = match redis::Client::open(kv_uri) {
        //     Ok(c) => c,
        //     Err(_) => return Err(
        //         HandleError::CacheConnectionError(
        //             "Failed to connect to Redis".to_string()
        //         )
        //     )
        // };
        // let mut multiplexed = match client.get_multiplexed_tokio_connection()
        // // let mut multiplexed = match client.get_multiplexed_tokio_connection()
        //     .await {
        //     Ok(m) => m,
        //     Err(_) => return Err(
        //         HandleError::CacheConnectionError(
        //             "Failed to get multiplexed connection".to_string()
        //         )
        //     )
        //     };
        // multiplexed.set_response_timeout(std::time::Duration::from_secs(5));
        Ok(AppState { 
            // tx: HashMap::new(),
            authed_tx: HashMap::new(),
            db_client,
            // kv_client: multiplexed,
            argon: Argon2::default(),
            client: reqwest::Client::new(),
        })
    }

    pub async fn get_tx(&self, id: Uuid) -> Result<MsgChan> {
        if let Some(tx) = self.authed_tx.get(&id) {
            return Ok(tx.clone());
        } else {
            return Err(HandleError::NoUserFound(
                "User not found".to_string()
            ));
        }
    }

    /// Clean up/close the connection from server to a client recognized by uid.
    /// token, this token is purely server-side and has no meaning to the client.
    pub async fn cleanup(&mut self, session_token: Uuid) -> Result<()> {
        debug!("Cleaning up connection for session token: {}", session_token.to_string());
        // let token_str = session_token.to_string();
        // let user_id: String = match client.get(token_str).await {
        //     Ok(u) => u,
        //     Err(e) => return Err(
        //         HandleError::CacheError(e)
        //     )
        // };
        // let user_id = match Uuid::parse_str(&user_id) {
        //     Ok(u) => u,
        //     Err(_) => return Err(
        //         HandleError::HandleError(
        //             "Failed to parse user id".to_string()
        //         )
        //     )
        // };
        // debug!("Cleaning up user id: {} for token: {}", user_id.to_string(), session_token.to_string());
        self.authed_tx.remove(&session_token);

        Ok(())
    }
}
