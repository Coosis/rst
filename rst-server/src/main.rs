use std::net::SocketAddr;
use std::ops::ControlFlow;
use std::sync::Arc;
use std::time::SystemTime;
use futures::{StreamExt, SinkExt};
use axum::body::Bytes;
use axum::extract::{ConnectInfo, Query, State};
use axum::response::IntoResponse;
use axum::routing::any;
use axum::Router;
use axum::extract::ws::{self, Message, WebSocket};
use axum_extra::headers::UserAgent;
use axum_extra::TypedHeader;
use jsonwebtoken::{decode, DecodingKey, Validation};
use lib::jwt::Jwt;
// use server::db_entry::DBEntries;
use tokio;
use tokio::sync::{Mutex, MutexGuard};
use tokio_util::sync::CancellationToken;
use tracing::{info, debug, warn, error};
use lib;
use lib::Uuid;
use lib::comm::{ClientInstruct, ClientMessage};
use lib::comm::client_instruct::{
    ClientConnectRequest, ClientConnectResponse, LoginRequest, RegisterRequest, SendMessage, ShowInvitesRequest, ShowMetadataRequest
};

mod handlers;
use handlers::*;
mod auth;
use auth::*;
mod state;
use state::AppState;
mod util;
use util::*;
mod db;
use db::*;
// mod cache;
// use cache::*;
mod msg_chan;
use msg_chan::*;

type SharedState = Arc<Mutex<AppState>>;
type LockedState<'a> = MutexGuard<'a, AppState>;
// type MsgChan = tokio::sync::mpsc::UnboundedSender<Message>;
type Result<T> = std::result::Result<T, HandleError>;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    debug!("Initializing server");
    let state: SharedState = Arc::new(Mutex::new(AppState::new(
                MONGODB_URI, // Db uri
                CACHE_URL    // Cache uri
                ).await?));
    let app = Router::new()
        .route("/ws", any(ws_handler))
        .with_state(state)
        .into_make_service_with_connect_info::<SocketAddr>();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await?;
    axum::serve(listener, app)
        .await?;
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Params {
    token: Option<String>,
}

async fn ws_handler(
    ws: ws::WebSocketUpgrade,
    user_agent: Option<TypedHeader<UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(params): Query<Params>,
    State(state): State<SharedState>,
    ) -> Result<impl IntoResponse> {
    let user_agent = if let Some(user_agent) = user_agent {
        user_agent.0.to_string()
    } else {
        "unknown".to_string()
    };
    info!("User-Agent: {}", user_agent);
    info!("Connection from: {}", addr);
    let token = match params.token {
        Some(token) => token,
        None => {
            String::from("None")
        }
    };

    // token extraction and verification
    info!("token: {}", &token);
    let token: Jwt = match decode::<Jwt>(
        &token,
        &DecodingKey::from_secret(SECRET),
        &Validation::new(jsonwebtoken::Algorithm::HS256)
        ){
        Ok(token) => {
            info!("Token verified");
            token.claims
        }
        Err(e) => {
            error!("Token verification failed: {:?}", e);
            return Err(HandleError::HandleError(
                "Token verification failed".to_string()
            ));
        }
    };
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64;
    if token.exp < now {
        debug!("Token expired");
        return Err(HandleError::TokenExpired);
    }

    debug!("upgrading to websocket");
    Ok(ws.on_upgrade(move |socket| handle_socket(
        socket,
        user_agent,
        addr,
        token,
        state
    )))
}

async fn handle_socket(
    mut socket: WebSocket,
    // TODO: logging user
    _user: String,
    addr: SocketAddr,
    token: Jwt,
    state: SharedState,
    ) {
    if socket
        .send(Message::Ping(Bytes::from_static(&[1, 2, 3])))
        .await
        .is_ok()
    {
        info!("Pinged {}...", addr);
    } else {
        info!("Could not send ping to {}!", addr);
        // no Error here since the only thing we can do is to close the connection.
        // If we can not send messages, there is no way to salvage the statemachine anyway.
        return;
    }

    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Message>();

    let chan = MsgChan::new(
        token.clone(),
        tx,
    );
    // map uid to websocket channel
    {
        let mut state = state.lock().await;
        match state.register_client(chan.clone()) {
            Ok(_) => {
                debug!("Registered client for uid: {}", &token.uid);
            }
            Err(e) => {
                warn!("Failed to register client: {:?}", e);
                return;
            }
        }
    }

    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            match sender.send(msg).await {
                Ok(_) => {
                    debug!("Sent message to {}", addr);
                }
                Err(e) => {
                    warn!("Failed to send message to {}: {:?}", addr, e);
                    break;
                }
            }
        }

        rx.close();
    });

    let token_cloned = token.clone();
    let state_cloned = state.clone();
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if process_message (
                token_cloned.clone(),
                state_cloned.clone(),
                msg,
                ).await.is_break() {
                return;
            }
        }
    });
    
    tokio::select! {
        _ = recv_task => {
            debug!("receiver task finished");
        }
        _ = send_task => {
            debug!("sender task finished");
        }
    }

    {
        let mut state = state.lock().await;
        let _ = state.cleanup(chan);
    }
}

async fn process_message(
    token: Jwt,
    state: SharedState,
    // tx: &MsgChan,
    msg: Message,
) -> ControlFlow<()> {
    match msg {
        Message::Binary(d) => {
            info!("Received {} bytes: {:?}", d.len(), d);
            // serde_json
            let cm: lib::comm::ClientMessage = match serde_json::from_slice(&d) {
                Ok(c) => c,
                Err(e) => {
                    warn!("Failed to parse message: {:?}", e);
                    return ControlFlow::Continue(());
                }
            };
            match handle(
                state,
                token,
                cm).await {
                Ok(_) => {
                    info!("Handled message");
                }
                Err(e) => {
                    warn!("Failed to handle message: {:?}", e);
                    return ControlFlow::Continue(());
                }
            }

        }
        Message::Text(t) => {
            info!("Received str: {t:?}");
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                info!("Received close: code: {}, reason: {}",
                    cf.code, cf.reason);
            } else {
                warn!("Received close without CloseFrame");
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            debug!("Received pong: {:?}", v);
        }
        // You should never need to manually handle Message::Ping, as axum's websocket library
        // will do so for you automagically by replying with Pong and copying the v according to
        // spec. But if you need the contents of the pings you can see them here.
        Message::Ping(v) => {
            debug!("Received ping: {:?}", v);
        }
    }
    ControlFlow::Continue(())
}

pub async fn handle(
    state: SharedState,
    token: Jwt,
    cm: ClientMessage
) -> Result<()> {
    let state = state.lock().await;
    match cm.instruct {
        ClientInstruct::Ack => handlers::handle_ack().await?,
        ClientInstruct::SendMessage => {
            let request = parse_inner::<SendMessage>(&cm.content)?;
            handlers::handle_send_message(
                token,
                &state, 
                request
            ).await?
        }
        ClientInstruct::ClientConnectRequest => {
            let request = parse_inner::<ClientConnectRequest>(&cm.content)?;
            connect_with(
                token,
                &state, 
                request
            ).await?;
            // TODO: storing logic
        }
        ClientInstruct::ClientConnectResponse => {
            let response = parse_inner::<ClientConnectResponse>(&cm.content)?;
            connect_response(
                token,
                state,
                response
            ).await?;
        }
        ClientInstruct::ListChats => {
            show_chats(token, &state)
                .await?;
        }
        ClientInstruct::ModifyChats => {
        }
        ClientInstruct::PollMessages => {
        }
        ClientInstruct::ShowInvitesRequest => {
            show_invites(
                token,
                &state,
            ).await?;
        }
        ClientInstruct::ShowMetadataRequest => {
            let request = parse_inner::<ShowMetadataRequest>(&cm.content)?;
            show_metadata(
                token,
                &state,
                request
            ).await?;
        }
        _ => {
            warn!("Unknow instruct: {:?}", cm.instruct);
        }
    }
    Ok(())
}

fn parse_inner<T>(inner: &[u8]) -> Result<T>
where
    T: serde::de::DeserializeOwned
{
    match serde_json::from_slice(inner) {
        Ok(i) => Ok(i),
        Err(e) => Err(
            HandleError::SerdeJsonError(e)
        )
    }
}
