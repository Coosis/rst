use lib::Uuid;
use lib::comm::ClientMessage;
use lib::comm::client_instruct::ClientConnectResponse;
use crate::{ClientError, LockedState};
use crate::Result;

pub async fn accept_invite(
    state: &LockedState<'_>,
    invite_id: String
    ) -> Result<()> {
    let inv_id = Uuid::parse_str(&invite_id).unwrap();
    // let token = Uuid::parse_str(&token).unwrap();
    let request = ClientConnectResponse::new(/* token,  */inv_id);
    let msg: ClientMessage = match request.try_into() {
        Ok(m) => m,
        Err(e) => return Err(ClientError::SerdeJsonError(e)),
    };
    if let Some(conn) = &state.connection {
        conn.send(msg).await?;
    } else {
        return Err(ClientError::NotConnected);
    }
    Ok(())
}
