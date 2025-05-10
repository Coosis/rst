use std::collections::{HashMap, HashSet};

use argon2::Argon2;
use axum::extract::ws::Message;
use lib::jwt::Jwt;
use lib::user::UserCredential;
use mongodb::Client;
// use redis::AsyncCommands;
use lib::Uuid;
use tokio::sync::mpsc::UnboundedSender;
use tracing::debug;

use crate::handlers::HandleError;
use crate::MsgChan;

type Result<T> = std::result::Result<T, HandleError>;

pub struct AppState {
    /// session id -> message channel
    // pub tx: HashMap<Uuid, MsgChan>,
    /// Used for message routing,
    /// user id -> message channel
    pub authed_tx: HashMap<Uuid, HashSet<MsgChan>>,
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
        _kv_uri: &str
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

    pub fn broadcast(&self, uid: Uuid, msg: Message) -> Result<()> {
        let tx = self.authed_tx.get(&uid).ok_or(
            HandleError::ChannelError(
                "Failed to get channel set".to_string()
            )
        )?;
        for chan in tx {
            chan.chan.send(msg.clone())?;
        }
        Ok(())
    }

    /// Used for registering a websocket channel for a single account
    pub fn register_client(
        &mut self,
        tx: MsgChan,
    ) -> Result<()> {
        debug!("Registering client");
        let uid = tx.jwt.uid;
        let set = match self.authed_tx.get_mut(&uid) {
            Some(set) => set,
            None => {
                let set = HashSet::new();
                self.authed_tx.insert(uid, set);
                self.authed_tx.get_mut(&uid).ok_or(
                    HandleError::ChannelError(
                        "Failed to get channel set".to_string()
                    )
                )?
            }
        };
        set.insert(tx.clone());

        Ok(())
    }

    /// Clean up/close that channel for that user, if that was the last channel, the entry of that
    /// user is removed
    pub fn cleanup(
        &mut self,
        chan: MsgChan
    ) -> Result<()> {
        let uid = chan.jwt.uid;
        debug!("Cleaning up one connection for user: {}", uid.to_string());
        let set = self.authed_tx.get_mut(&uid).ok_or(
            HandleError::ChannelError(
                "Failed to get channel set".to_string()
            )
        )?;
        set.remove(&chan);
        if set.is_empty() {
            self.authed_tx.remove(&uid);
        }

        Ok(())
    }

    /// Forcefully disconnect *every* WebSocket session for this `uid`.
    pub fn disconnect_user(&mut self, uid: Uuid) -> Result<()> {
        if let Some(channels) = self.authed_tx.remove(&uid) {
        }
        Ok(())
    }
}
