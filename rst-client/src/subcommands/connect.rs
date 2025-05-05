use std::ops::ControlFlow;
use std::str::from_utf8;

use lib::comm::client_instruct::LoginType;
use lib::comm::client_instruct::ShowMetadataRequest;
use lib::comm::server_instruct::PushMessage;
use lib::comm::server_instruct::ShowMetadataResponse;
use lib::content::ContentType;
use lib::user::MetadataQuery;
use tokio_tungstenite;
use tokio::io::AsyncBufReadExt;
use futures::SinkExt;
use futures::StreamExt;

use lib::comm::server_instruct::ServerConnectRequest;
use lib::comm::{
    ClientMessage,
    ServerInstruct,
    ServerMessage
};
use lib::comm::client_instruct::LoginRequest;
use tokio_tungstenite::tungstenite::Message;

const AUTH_SERVER: &str = "http://localhost:3345/auth";

use crate::parse_inner;
use crate::read_loop;
use crate::ClientError;
pub async fn connect(addr: String, sub: bool) 
    -> Result<(), ClientError> {
    // -asking for login credentials-----------------------------------
    if sub {
        println!("Subscribing to server...")
    }
    println!("addr: {}", addr);
    let mut buffered = tokio::io::BufReader::new(tokio::io::stdin());
    let delim = "=".repeat(20);
    println!("{}", delim);
    let mut email = String::new();
    let mut phone = String::new();

    println!("Email:");
    buffered.read_line(&mut email).await?;
    email = email.trim_end_matches('\n').to_string();
    email = email.trim_end_matches('\r').to_string();

    if email.is_empty() {
        println!("Phone:");
        buffered.read_line(&mut phone).await?;
        phone = phone.trim_end_matches('\n').to_string();
        phone = phone.trim_end_matches('\r').to_string();
    }

    if email.is_empty() && phone.is_empty() {
        return Err(ClientError::InvalidInput(
                "No user id, email, or phone provided"
                .to_string()
                ));
    }

    let email = if email.is_empty() {
        None
    } else { Some(email) };
    let phone = if phone.is_empty() {
        None
    } else { Some(phone) };

    println!("Password:");
    let mut passwd = String::new();
    buffered.read_line(&mut passwd).await?;
    passwd = passwd.trim_end_matches('\n').to_string();
    if passwd.is_empty() {
        return Err(ClientError::InvalidInput(
                "No password provided".to_string()
                ));
    }
    println!("{}", delim);

    // ----------------------------------------------------------------

    let client = reqwest::Client::new();

    let login_request = if let Some(email) = email.clone() {
        LoginRequest::new(
            LoginType::ByEmail,
            email,
            passwd
        )
    } else if let Some(phone) = phone.clone() {
        LoginRequest::new(
            LoginType::ByPhone,
            phone, 
            passwd
        )
    } else {
        return Err(ClientError::InvalidInput(
            "No user id, email, or phone provided".to_string()
        ));
    };
    let response = client.post(AUTH_SERVER)
        .json(&login_request)
        .send()
        .await?;
    println!("Response status code: {}", response.status());
    let token = response
        .text()
        .await?;
    println!("Token: {}", token);
    let addr = format!("{}?token={}", addr, token);

    let (ws_stream, response) = match tokio_tungstenite::connect_async(addr).await {
        Ok((stream, response)) => (stream, response),
        Err(e) => return Err(ClientError::ConnectionError(e.to_string())),
    };

    println!("Response status code: {}", response.status());
    let (mut ws_sink, ws_stream) = ws_stream.split();

    read_loop(ws_stream, async move |msg: ServerMessage| {
        let token = token.clone();
        match msg.instruct {
            ServerInstruct::LoginResponse => {
                println!("Login successful");
            }
            ServerInstruct::ServerConnectRequest => {
                println!("Connect request received: ");
                let msg = parse_inner::<ServerConnectRequest>(&msg.content);
                println!("{:?}", msg);
                let from = msg.from;
                let query = MetadataQuery::new(from);
                let public_data = client.get(format!(
                    "{}/get_metadata",
                    AUTH_SERVER,
                ))
                    .json(&query)
                    .send().await.unwrap()
                    .text().await.unwrap();
                println!("Public data: {}", public_data);
                let uid = msg.from;
                let query = ShowMetadataRequest::new(token, vec![uid]);
                let msg: ClientMessage = match query.try_into() {
                    Ok(msg) => msg,
                    Err(e) => {
                        eprintln!("Failed to convert request to message: {}", e);
                        return ControlFlow::Break(ClientError::SerdeJsonError(e));
                    }
                };
                let msg_bytes: Vec<u8> = match msg.try_into() {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        eprintln!("Failed to serialize message: {}", e);
                        return ControlFlow::Break(ClientError::SerdeJsonError(e));
                    }
                };
                let msg = Message::Binary(msg_bytes.into());
                let _ = ws_sink.send(msg).await;
            }
            ServerInstruct::ShowMetadataResponse => {
                println!("Show metadata response received: ");
                let msg = parse_inner::<ShowMetadataResponse>(&msg.content);
                for (i, r) in msg.data.iter().enumerate() {
                    println!("User {}: ", i);
                    println!("Associated user id: {:?}", r.id);
                    println!("Associated user email: {:?}", r.email);
                    println!("Associated user phone: {:?}", r.phone);
                    println!();
                }
            }
            ServerInstruct::PushMessage => {
                println!("Push message received: ");
                let msg = parse_inner::<PushMessage>(&msg.content).message;
                println!("Timestamp: {:.3}", msg.timestamp);
                println!("From: {:?}", msg.from);
                println!("To: {:?}", msg.to);
                for content in msg.contents {
                    match content.ctype {
                        ContentType::Text => {
                            println!("{}", from_utf8(&content.data).unwrap());
                        }
                        _ => {
                            println!("Other content type: {}", content.ctype);
                        }
                    }
                }
            }
            _ => {
                println!("Unexpected message type");
            }
        };

        ControlFlow::Continue(())
    }).await?;
    let a: Box<str> = "hello".into();
    Ok(())
}
