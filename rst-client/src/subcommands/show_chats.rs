use lib::comm::client_instruct::ListChats;
use lib::comm::ClientMessage;
use crate::ClientError;
use crate::LockedState;
pub async fn show_chats(state: &LockedState<'_>) 
    -> Result<(), ClientError> {
    let req: ClientMessage = ListChats::new().try_into()?;
    if let Some(conn) = &state.connection {
        conn.send(req).await?;
    } else {
        return Err(ClientError::NotConnected);
    }
    Ok(())
}
