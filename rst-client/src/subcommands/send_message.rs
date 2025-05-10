use lib::comm::client_instruct::SendMessage;
use lib::content::Content;
use lib::message::MessageBuilder;
use lib::comm::ClientMessage;
use lib::Uuid;

use crate::ClientError;
use crate::LockedState;

pub async fn send_message(
    state: &LockedState<'_>,
    chat_id: Uuid,
    message: String,
    ) 
    -> Result<(), ClientError> {

    let message = MessageBuilder::new(chat_id)
        .push(Content::new_text(message));
    let req: ClientMessage = SendMessage::new(message).try_into()?;

    if let Some(conn) = &state.connection {
        conn.send(req).await?;
    } else {
        return Err(ClientError::NotConnected);
    }
    Ok(())
}

