use axum::extract::ws::Message;
use mongodb::{self, bson};
use tokio::sync::MutexGuard;
use tracing::debug;

use lib::comm::server_instruct::LoginResponse;
use lib::jwt::Jwt;

use crate::Result;
use crate::message_util::MessageExt;
use crate::state::AppState;
use crate::handlers::HandleError;


/// Handle login requests:
/// *1. map uid to websocket channel
///
/// Returns `LoginResponse` as a Axum Websocket message
pub async fn login_handler(
    mut state: MutexGuard<'_, AppState>,
    tx: tokio::sync::mpsc::UnboundedSender<Message>,
    token: Jwt,
    ) -> Result<()> {
    debug!("Handling login request");
    let uid = token.uid;
    state.authed_tx.insert(uid, tx.clone());
    let msg = LoginResponse::new().try_into_ws_msg()?;
    match tx.send(msg) {
        Ok(_) => {
            debug!("Login response sent");
        },
        Err(e) => return Err(
            HandleError::SendError(e.to_string())
        )
    }
    debug!("Login request handled");
    Ok(())
}
