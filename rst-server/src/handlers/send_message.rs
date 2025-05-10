use bson::doc;
use lib::chat::Chat;
use lib::comm::client_instruct::SendMessage;
use lib::comm::server_instruct::PushMessage;
use lib::jwt::Jwt;
use lib::message::Message;
use lib::Uuid;
use tracing::debug;

use crate::message_util::MessageExt;
use crate::{find_one, insert_one, HandleError, LockedState, Result, TB_MESSAGES};
use crate::TB_CHATS;

pub async fn handle_send_message(
    token: Jwt,
    state: &LockedState<'_>,
    req: SendMessage,
    ) -> Result<()> {
    debug!("Handling send message");

    let db_client = state.db_client.clone();
    let filter = doc! {
        "id": req.message.to,
        "members": token.uid,
    };
    let chat: Chat = find_one(
        &db_client,
        TB_CHATS,
        filter, 
        HandleError::NoUserFound("chat not found".to_string())
        ).await?;

    let members = chat.members;
    let msg_id = Uuid::new_v4();
    let msg = req.message.build(msg_id);
    for m in members {
        // if m == token.uid {
        //     continue;
        // }
        if !state.authed_tx.contains_key(&m) {
            continue;
        }
        let msg = msg.clone();
        let msg = PushMessage::new(msg)
            .try_into_ws_msg()?;
        state.broadcast(m, msg)?;

    }
    insert_one::<Message>(
        &db_client,
        TB_MESSAGES,
        msg,
    ).await?;

    debug!("Handling send message done");
    Ok(())
}
