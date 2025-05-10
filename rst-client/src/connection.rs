use std::ops::ControlFlow;
use std::str::from_utf8;
use futures::{SinkExt, StreamExt};
use lib::comm::client_instruct::ShowMetadataRequest;
use lib::comm::server_instruct::{LoginResponse, PushMessage, ServerConnectRequest, ShowChatsResponse, ShowInvitesResponse};
use lib::comm::{ClientMessage, ServerInstruct, ServerMessage};
use lib::content::ContentType;
use lib::user::{MetadataQuery, PublicUserCredential};
use reedline::ExternalPrinter;
use reqwest::StatusCode;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::Message;
use tokio_util::sync::CancellationToken;

use crate::{error::ClientError, read_loop, Result};
use crate::{parse_inner, SharedState, AUTH_SERVER, METADATA_SERVER};

pub struct Connection {
    shutdown: CancellationToken,
    read_task: JoinHandle<()>,
    write_task: JoinHandle<()>,
    tx: mpsc::Sender<Message>,
}

impl Connection {
    pub async fn connect(
        state: &SharedState,
        printer: ExternalPrinter<String>,
    ) -> Result<Self> {
        let shared = state.clone();
        let (server_url, token) = {
            let state = state.lock().await;
            let token = state.token.clone()
                .ok_or(ClientError::InvalidInput(
                    "No token provided".to_string(),
                ))?;
            (state.server.clone(), token)
        };
        {
            let mut state = shared.lock().await;
            state.disconnect();
        }
        let addr = format!("{}?token={}", server_url, token);
        let (ws_stream, response) = match tokio_tungstenite::connect_async(addr).await {
            Ok((stream, response)) => (stream, response),
            Err(e) => {
                return Err(ClientError::ConnectionError(
                        format!("Failed to connect to websocket server: {}", e).to_string()
                ))
            }
        };
        if response.status() != StatusCode::SWITCHING_PROTOCOLS {
            println!("Failed to connect to server: {}", response.status());
            return Err(ClientError::ConnectionError(
                    format!("Failed to connect to server: {}", response.status()).to_string()
            ));
        }
        let (mut ws_sink, ws_stream) = ws_stream.split();
        let (tx, mut rx) = mpsc::channel::<Message>(50);

        let shutdown = CancellationToken::new();
        let rshutdown = shutdown.clone();
        let wshutdown = shutdown.clone();

        let rstate = shared.clone();
        let rprinter = printer.clone();
        let rprinter_inner = rprinter.clone();
        let token_clone = token.clone();
        let tx_clone = tx.clone();
        let handler = async move |msg: ServerMessage| {
            match msg.instruct {
                ServerInstruct::PushMessage => {
                    let msg = parse_inner::<PushMessage>(&msg.content).message;
                    let _ = rprinter_inner.print(format!("time: {:4}", msg.timestamp));
                    let _ = rprinter_inner.print(format!("from: {}", msg.from));
                    for c in msg.contents {
                        if c.ctype == ContentType::Text {
                            let _ = rprinter_inner.print(format!("text: {}", from_utf8(&c.data).unwrap()));
                        } else {
                            let _ = rprinter_inner.print(format!("Unsupported content: {}", c.ctype));
                        }
                    }
                }
                ServerInstruct::LoginResponse => {
                    let msg: LoginResponse = parse_inner::<LoginResponse>(&msg.content);
                }
                ServerInstruct::ShowInvitesResponse => {
                    let msg: ShowInvitesResponse = parse_inner::<ShowInvitesResponse>(&msg.content);
                    let client = reqwest::Client::new();
                    for invite in msg.invites {
                        let _ = rprinter_inner.print(format!("Invite id: {}", invite.id));
                        let _ = rprinter_inner.print(format!("Invite from: {}", invite.from));
                        let query = MetadataQuery::new(invite.from);
                        let public_data = client.get(METADATA_SERVER)
                            .json(&query)
                            .send().await.unwrap()
                            .text().await.unwrap();
                        let data: PublicUserCredential = serde_json::from_str(&public_data).unwrap();
                        let _ = rprinter_inner.print(format!("His email: {}", data.email.unwrap_or_default()));
                        let _ = rprinter_inner.print(format!("His phone: {}", data.phone.unwrap_or_default()));
                    }
                }
                ServerInstruct::ShowMetadataResponse => {
                    let _ = rprinter_inner.print(format!("Show metadata response: {:?}", msg));
                }
                ServerInstruct::ServerConnectRequest => {
                    let msg = parse_inner::<ServerConnectRequest>(&msg.content);
                    let _ = rprinter_inner.print(format!("Received invite: {}", msg.name));
                    let _ = rprinter_inner.print(format!("Description {}", msg.description));
                    let _ = rprinter_inner.print(format!("From {}", msg.from));

                    let from = msg.from;
                    let query = MetadataQuery::new(from);
                    let client = reqwest::Client::new();
                    let public_data = client.get(METADATA_SERVER)
                        .json(&query)
                        .send().await.unwrap()
                        .text().await.unwrap();
                    let data: PublicUserCredential = serde_json::from_str(&public_data).unwrap();
                    let _ = rprinter_inner.print(format!("His email: {}", data.email.unwrap_or_default()));
                    let _ = rprinter_inner.print(format!("His phone: {}", data.phone.unwrap_or_default()));
                }
                ServerInstruct::ShowChatsResponse => {
                    let msg = parse_inner::<ShowChatsResponse>(&msg.content);
                    for chat in msg.chats {
                        let _ = rprinter_inner.print(format!("Chat id: {}", chat.id));
                        let _ = rprinter_inner.print(format!("Chat name: {}", chat.name));
                        let _ = rprinter_inner.print(format!("Chat description: {}", chat.description));
                    }
                }
                _ => {
                    let _ = rprinter_inner.print(format!("Unexpected message type"));
                }
            };
            return ControlFlow::Continue(());
        };
        let read_task = tokio::spawn(async move {
            tokio::select! {
                biased;
                _ = rshutdown.cancelled() => {
                    let _ = rprinter.print(format!("Read task cancelled"));
                    return;
                }
                _ = read_loop(ws_stream, handler) => {}
            };
            rshutdown.cancel();
            let mut state = rstate.lock().await;
            state.disconnect();
        });

        let wstate = shared.clone();
        let wprinter = printer.clone();
        let wprinter_inner = wprinter.clone();
        let mut write_future = async move || {
            while let Some(msg) = rx.recv().await {
                let msg = Message::Binary(msg.into());
                if let Err(e) = ws_sink.send(msg).await {
                    let _ = wprinter_inner.print(format!("Failed to send message: {}", e));
                }
            }
        };
        let write_task = tokio::spawn(async move {
            tokio::select! {
                biased;
                _ = wshutdown.cancelled() => {
                    let _ = wprinter.print(format!("Write task cancelled"));
                    return;
                }
                _ = write_future() => {
                    let _ = wprinter.print(format!("Write task finished"));
                }
            };
            wshutdown.cancel();
            let mut state = wstate.lock().await;
            state.disconnect();
        });

        {
            let mut state = shared.lock().await;
            state.connected = true;
            state.token = Some(token);
        }
        Ok(Self {
            shutdown,
            read_task,
            write_task,
            tx,
        })
    }

    /// Send a message to the server
    pub async fn send(
        &self,
        msg: ClientMessage,
    ) -> Result<()> {
        let msg: Vec<u8> = msg.try_into()?;
        let msg = Message::Binary(msg.into());
        if let Err(e) = self.tx.send(msg).await {
            return Err(ClientError::ConnectionError(
                format!("Failed to send message: {}", e).to_string(),
            ));
        }
        Ok(())
    }

    /// Remember to update connection state separately for application
    pub async fn shutdown(self) {
        self.shutdown.cancel();
        let _ = self.read_task.await;
        let _ = self.write_task.await;
    }
}
