use std::ops::ControlFlow;
use lib::comm::client_instruct::ShowInvitesRequest;
use lib::comm::server_instruct::ShowInvitesResponse;
use lib::comm::ServerMessage;
use tokio_tungstenite;
use futures::SinkExt;
use futures::StreamExt;
use lib::comm::{
    ClientMessage,
    ServerInstruct,
};
use crate::parse_inner;
use crate::read_loop;
use crate::ClientError;
pub async fn show_invites(addr: &str/* token: Uuid */) 
    -> Result<(), ClientError> {
    let (ws_stream, response) = match tokio_tungstenite::connect_async(addr).await {
        Ok((stream, response)) => (stream, response),
        Err(e) => return Err(ClientError::ConnectionError(e.to_string())),
    };

    println!("Response status code: {}", response.status());
    let (mut ws_sink, ws_stream) = ws_stream.split();

    let req: ClientMessage = ShowInvitesRequest::new().try_into()?;
    let json_bytes: Vec<u8> = match req.try_into() {
        Ok(bytes) => bytes,
        Err(e) => return Err(ClientError::SerdeJsonError(e)),
    };
    let msg = tokio_tungstenite::tungstenite::Message::Binary(json_bytes.into());
    let _ = ws_sink.send(msg).await;

    read_loop(ws_stream, async move |msg: ServerMessage| {
        match msg.instruct {
            ServerInstruct::ShowInvitesResponse => {
                println!("Show invites response received: ");
                let msg = parse_inner::<ShowInvitesResponse>(&msg.content);
                for invite in msg.invites {
                    println!("{:?}", invite);
                }
                ControlFlow::Break(ClientError::None)
            }
            _ => {
                println!("Unexpected message type: {:?}", msg.instruct);
                ControlFlow::Continue(())
            }
        }
    }).await?;
    Ok(())
}
