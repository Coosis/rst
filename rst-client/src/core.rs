use std::ops::ControlFlow;

use futures::Stream;
use futures_util::StreamExt;
use lib::comm::ServerMessage;
use tokio_tungstenite::tungstenite::{self, Message};
use tracing::debug;
use crate::ClientError;
pub async fn read_loop(
    mut ws_stream: impl Stream<Item = Result<Message, tungstenite::Error>> + Unpin,
    mut bin_handler: impl (AsyncFnMut(ServerMessage)->ControlFlow<ClientError>)
) -> Result<(), ClientError> {
    loop {
        let msg = ws_stream.next().await;
        let msg = match msg {
            Some(Ok(m)) => m,
            Some(Err(e)) => return Err(ClientError::ConnectionError(e.to_string())),
            None => return Err(ClientError::ConnectionError(
                    "Connection closed".to_string()
            )),
        };

        let msg = match msg {
            tokio_tungstenite::tungstenite::Message::Binary(b) => b,
            tokio_tungstenite::tungstenite::Message::Close(_) => {
                return Err(ClientError::ConnectionError(
                        "Connection closed".to_string()
                ))
            }
            tokio_tungstenite::tungstenite::Message::Text(t) => {
                println!("Received text message: {}", t);
                continue;
            }
            tokio_tungstenite::tungstenite::Message::Ping(_) => {
                println!("Received ping");
                continue;
            }
            tokio_tungstenite::tungstenite::Message::Pong(_) => {
                println!("Received pong");
                continue;
            }
            _ => return Err(ClientError::ConnectionError(
                    "Unexpected message type".to_string()
            )),
        };
        let msg: ServerMessage = serde_json::from_slice(&msg).unwrap();
        match bin_handler(msg).await {
            ControlFlow::Continue(()) => {
            }
            ControlFlow::Break(e) => {
                debug!("Error in read loop: {:?}", e);
                break;
            }
        }
    }
    Ok(())
}
