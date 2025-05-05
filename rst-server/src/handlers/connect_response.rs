use bson::doc;
use lib::chat::Chat;
use lib::comm::client_instruct::ClientConnectResponse;
use lib::invite::Invitation;
use lib::util::BsonDocExt;
use lib::Uuid;
use tokio::sync::MutexGuard;
use tracing::debug;

use crate::state::AppState;
use crate::{find_one_and_delete, update_one, HandleError, Result};
use crate::{TB_CHATS, TB_INVITES};

pub async fn connect_response(
    uid: Uuid,
    state: MutexGuard<'_, AppState>,
    req: ClientConnectResponse,
    ) -> Result<()> {
    debug!("Handling connect_response");
    let db_client = state.db_client.clone();
    let invite_filter = doc! {
        "id": req.invite_id,
        "to": uid.clone(),
    };
    let invitation: Invitation = find_one_and_delete(
        &db_client,
        TB_INVITES,
        invite_filter
        ).await
        .decode(HandleError::NoInvitationFound("no invitation found".to_string()))?;
    let roomid = invitation.chat_id;
    debug!("Invitation found: {:?}", invitation);
    let chat_filter = doc! { "id": roomid, };
    let update = doc! { "$addToSet": { "members": uid.clone() } };
    update_one::<Chat>(
        &db_client,
        TB_CHATS,
        chat_filter.clone(),
        update.clone()
    ).await?;

    debug!("Handled connect_response");
    Ok(())
}
