use lib::Uuid;
use tokio;
use tokio_tungstenite::tungstenite::protocol::Message;
use std::ops::ControlFlow;
mod subcommands;
use subcommands::*;
mod error;
use crate::error::ClientError;
use clap::Parser;
use clap::Subcommand;

mod core;
use core::*;

const SERVER: &str = "ws://127.0.0.1:3000/ws";

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    subcommand: Commands,
}

/// Subcommands used by rst-client
#[derive(Debug)]
#[derive(Subcommand)]
enum Commands {
    /// Connect to server
    Connect {
        /// Address to connect to
        addr: String, 
        #[arg(short, long, value_name = "sub", default_value_t = false)]
        sub: bool,
    },

    /// Register a user
    Register {
        /// Address to connect to
        addr: String, 
        #[arg(long, value_name = "phone")]
        phone: Option<String>,
        #[arg(long, value_name = "email")]
        email: Option<String>,
        #[arg(short, long, value_name = "username")]
        username: String,
        #[arg(short, long, value_name = "passwd")]
        passwd: String,
    },

    /// Send a request to a user to chat
    SendRequest {
        /// Address to connect to
        addr: String, 
        #[arg(short, long, value_name = "token")]
        token: String,
        #[arg(long, value_name = "email")]
        email: Option<String>,
        #[arg(long, value_name = "phone")]
        phone: Option<String>,
        #[arg(long, value_name = "id")]
        id: Option<String>,

        #[arg(long, value_name = "name")]
        name: String,
        #[arg(long, value_name = "description")]
        description: String,
    },

    ShowInvites {
        /// Address to connect to
        addr: String,
        #[arg(short, long, value_name = "token")]
        token: String,
    },

    AcceptInvite {
        /// Address to connect to
        addr: String,
        #[arg(short, long, value_name = "token")]
        token: String,
        #[arg(short, long, value_name = "invite_id")]
        invite_id: String,
    },

    SendMessage {
        /// Address to connect to
        addr: String,
        #[arg(short, long, value_name = "token")]
        token: String,
        #[arg(short, long, value_name = "chat_id")]
        chat_id: String,
        #[arg(short, long, value_name = "message")]
        message: String,
    }
}

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    let cli = Cli::parse();
    match cli.subcommand {
        Commands::Connect { addr, sub } => connect(addr, sub).await,
        Commands::Register { addr, phone, email, username, passwd } 
            => register(addr, phone, email, username, passwd).await,
        Commands::SendRequest { addr, token, email, phone, id, name, description }=> {
            let addr = format!("{}?token={}", addr, token);
            send_chat_request(&addr, email, phone, id, name, description).await
        },
        Commands::ShowInvites { addr, token } => {
            let addr = format!("{}?token={}", addr, token);
            show_invites(&addr).await
        },
        Commands::AcceptInvite { addr, token, invite_id } => {
            let addr = format!("{}?token={}", addr, token);
            accept_invite(&addr, invite_id).await
        },
        Commands::SendMessage { addr, token, chat_id, message } => {
            let addr = format!("{}?token={}", addr, token);
            let chat_id = Uuid::parse_str(&chat_id)
                .map_err(|_| ClientError::InvalidInput("Invalid chat id".to_string()))?;
            send_message(&addr, chat_id, message).await
        }
        // _ => {
        //     println!("Not implemented");
        //     Ok(())
        // }
    //     // Commands::Login { addr, email, password } 
    //     //     => login(&addr, email, password).await,
    }
}

fn process_message(msg: Message) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            println!(">>> got str: {t:?}");
        }
        Message::Binary(d) => {
            println!(">>> got {} bytes: {:?}", d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> got close with code {} and reason `{}`",
                    cf.code, cf.reason
                );
            } else {
                println!(">>> somehow got close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            println!(">>> got pong with {v:?}");
        }
        // Just as with axum server, the underlying tungstenite websocket library
        // will handle Ping for you automagically by replying with Pong and copying the
        // v according to spec. But if you need the contents of the pings you can see them here.
        Message::Ping(v) => {
            println!(">>> got ping with {v:?}");
        }

        Message::Frame(_) => {
            unreachable!("This is never supposed to happen")
        }
    }
    ControlFlow::Continue(())
}


pub fn parse_inner<T>(inner: &[u8]) -> T
where
    T: serde::de::DeserializeOwned
{
    serde_json::from_slice(inner).unwrap()
}
