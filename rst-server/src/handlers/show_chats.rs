use bson::doc;
use lib::chat::Chat;
use lib::jwt::Jwt;

use crate::util::message_util::MessageExt;
use crate::{find_many, LockedState, Result, TB_CHATS};

pub async fn show_chats(
    token: Jwt,
    state: &LockedState<'_>,
) -> Result<()> {
    tracing::debug!("Handling show_invites");
    let db_client = state.db_client.clone();
    let filter = doc! {
        "members": token.uid
    };
    let chats: Vec<Chat> = find_many(
        &db_client,
        TB_CHATS, 
        filter
        ).await?;
    let msg = lib::comm::server_instruct::ShowChatsResponse::new(chats)
        .try_into_ws_msg()?;
    
    state.broadcast(token.uid, msg)?;
    tracing::debug!("Sent chats to channel");
    Ok(())
}
