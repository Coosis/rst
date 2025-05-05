use lib::jwt::Jwt;
use lib::user::UidQuery;
use lib::Uuid;
use lib::chat::Chat;
use lib::invite;
use lib::comm::client_instruct::ClientConnectRequest;
use lib::comm::server_instruct::ServerConnectRequest;
use tokio::sync::MutexGuard;
use tracing::debug;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::insert_one;
use crate::message_util::MessageExt;
use crate::state::AppState;
use crate::AUTH_SERVER;
use crate::{MsgChan, Result, HandleError};
use crate::{TB_CHATS, TB_USERS, TB_INVITES};

pub async fn connect_with(
    state: MutexGuard<'_, AppState>,
    token: Jwt,
    req: ClientConnectRequest,
    ) -> Result<()> {
    debug!("Handling connect_with");
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs() as i64;

    let sender = token.uid;

    let db_client = state.db_client.clone();
    let chatid = Uuid::new_v4();
    debug!("creating chat with id: {}", chatid);
    let mut invites = vec![];
    for u in &req.to {
        let recv_id = match u.id {
            Some(u) => u,
            None => {
                let query = UidQuery::new(
                    u.email.clone(),
                    u.phone.clone()
                );
                let get_uid_url = format!(
                    "{}/get_uid",
                    AUTH_SERVER,
                );
                let response = state.client.post(&get_uid_url)
                    .json(&query)
                    .send()
                    .await?;
                debug!("Response status code: {}", response.status());
                response
                    .json::<Uuid>()
                    .await?
            }
        };
        let tx = state.get_tx(recv_id).await.ok();
        invites.push(invite_user(
            tx,
            &req,
            &db_client,
            sender,
            recv_id,
            chatid.clone()
        ).await?);
    }

    let chat = Chat::new(
        chatid,
        req.name,
        req.description,
        now,
        now,
        vec![sender],
        vec![]
    );
    insert_one(
        &db_client,
        TB_CHATS,
        chat)
        .await?;
    debug!("Handling connect_with done");

    Ok(())
}

async fn invite_user(
    tx: Option<MsgChan>,
    req: &ClientConnectRequest,
    db_client: &mongodb::Client,
    sender: Uuid, // sender's uid
    receiver: Uuid,
    chat_id: Uuid
) -> Result<invite::Invitation> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs() as i64;

    // getting receiver
    let inv = lib::invite::Invitation::new(sender, receiver, chat_id);
    insert_one::<lib::invite::Invitation>(
        &db_client,
        TB_INVITES,
        &inv
        ).await?;

    // stream the invite if the receiver is online
    if let Some(tx) = tx {
        let msg = ServerConnectRequest::new(
            sender,
            now,
            req.name.clone(),
            req.description.clone())
            .try_into_ws_msg()?;
        let _ = tx.send(msg);
    }
    Ok(inv)
}
