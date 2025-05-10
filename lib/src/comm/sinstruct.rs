// server-client communication
use serde::{Deserialize, Serialize};

/// Server to client communication
#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum ServerInstruct {
    /// Server sends this message to the client to acknowledge 
    /// that some message has been received.
    Ack = 0,
    /// Server sends this message to the client to push 
    /// a message to the chat window.
    PushMessage = 1,
    /// The server sends this message to the client to inform 
    /// that there's a chat request from a new client.
    ServerConnectRequest = 2,
    /// The server sends this message to the client to inform
    /// the result of the previous chat request.
    ConnectResponse = 3,
    LoginResponse = 4,
    RegisterResponse = 5,
    ShowInvitesResponse = 6,
    ShowMetadataResponse = 7,
    ShowChatsResponse = 8,

    /// Unknown message
    Unknown = -1,
}

impl Serialize for ServerInstruct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_i32(self.clone() as i32)
    }
}

impl<'de> Deserialize<'de> for ServerInstruct {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let instruct = i32::deserialize(deserializer)?;
        match instruct {
            0 => Ok(ServerInstruct::Ack),
            1 => Ok(ServerInstruct::PushMessage),
            2 => Ok(ServerInstruct::ServerConnectRequest),
            3 => Ok(ServerInstruct::ConnectResponse),
            4 => Ok(ServerInstruct::LoginResponse),
            5 => Ok(ServerInstruct::RegisterResponse),
            6 => Ok(ServerInstruct::ShowInvitesResponse),
            7 => Ok(ServerInstruct::ShowMetadataResponse),
            8 => Ok(ServerInstruct::ShowChatsResponse),
            _ => Ok(ServerInstruct::Unknown),
        }
    }
}
