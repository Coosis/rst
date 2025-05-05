use futures::{SinkExt, StreamExt};

use lib::Uuid;
use lib::comm::ClientMessage;
use lib::comm::client_instruct::ClientConnectResponse;
use crate::ClientError;
type Result<T> = std::result::Result<T, ClientError>;

pub async fn accept_invite(
    addr: &str,
    invite_id: String
    ) ->Result<()> {
    let (ws_stream, response) = match tokio_tungstenite::connect_async(addr).await {
        Ok((stream, response)) => (stream, response),
        Err(e) => return Err(ClientError::ConnectionError(e.to_string())),
    };

    println!("Response status code: {}", response.status());
    let (mut ws_sink, _) = ws_stream.split();
    let inv_id = Uuid::parse_str(&invite_id).unwrap();
    // let token = Uuid::parse_str(&token).unwrap();
    let request = ClientConnectResponse::new(/* token,  */inv_id);
    let msg: ClientMessage = match request.try_into() {
        Ok(m) => m,
        Err(e) => return Err(ClientError::SerdeJsonError(e)),
    };
    let json_bytes: Vec<u8> = match serde_json::to_vec(&msg) {
        Ok(bytes) => bytes,
        Err(e) => return Err(ClientError::SerdeJsonError(e)),
    };
    let msg = tokio_tungstenite::tungstenite::Message::Binary(json_bytes.into());
    let _ = ws_sink.send(msg).await;
    Ok(())
}
