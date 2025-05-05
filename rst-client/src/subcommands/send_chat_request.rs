use std::str::FromStr;

use crate::ClientError;
use lib::{user::PublicUserCredential, Uuid};
use lib::comm::ClientMessage;
use lib::comm::client_instruct::ClientConnectRequest;
use futures::{SinkExt, StreamExt};
type Result<T> = std::result::Result<T, ClientError>;

pub async fn send_chat_request(
    addr: &str,

    email: Option<String>,
    phone: Option<String>,
    uid: Option<String>,

    name: String,
    description: String,
    ) ->Result<()> {
    if email.is_none() && phone.is_none() && uid.is_none() {
        return Err(ClientError::InvalidInput(
            "No user id, email, or phone provided".to_string()
        ));
    }
    let uid: Option<Uuid> = match uid {
        Some(u) => match Uuid::from_str(&u) {
            Ok(u) => Some(u),
            Err(_) => return Err(ClientError::InvalidInput("
                    Failed to parse user id".to_string()
            )),
        },
        None => None,
    };
    let to = PublicUserCredential::new(
        email, phone, uid, None);
    let request = ClientConnectRequest::new(
        vec![to],
        name,
        description);

    let (ws_stream, response) = match tokio_tungstenite::connect_async(addr).await {
        Ok((stream, response)) => (stream, response),
        Err(e) => return Err(ClientError::ConnectionError(e.to_string())),
    };

    println!("Response status code: {}", response.status());
    let (mut ws_sink, _) = ws_stream.split();

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
