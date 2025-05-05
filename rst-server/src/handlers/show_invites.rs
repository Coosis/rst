use bson::doc;
use lib::invite::Invitation;
use lib::Uuid;

use crate::util::message_util::MessageExt;
use crate::state::AppState;
use crate::{find_many, HandleError, MsgChan, Result};
use crate::TB_INVITES;

pub async fn show_invites(
    state: tokio::sync::MutexGuard<'_, AppState>,
    tx: &MsgChan,
    token: Uuid,
) -> Result<()> {
    tracing::debug!("Handling show_invites");
    let db_client = state.db_client.clone();
    let uid = token;
    let filter = doc! {
        "to": uid,
    };
    let invites: Vec<Invitation> = find_many(
        &db_client,
        TB_INVITES, 
        filter.clone()
        ).await?;
    let msg = lib::comm::server_instruct::ShowInvitesResponse::new(invites)
        .try_into_ws_msg()?;
    let _ = tx.send(msg)
        .map_err(|e| HandleError::SendError(e.to_string()))?;
    tracing::debug!("Sent invites to channel");
    Ok(())
}
