// server-client communication
use serde::{Deserialize, Serialize};

/// Client to server communication
#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum ClientInstruct {
    /// Client sends this message to the server to acknowledge 
    /// that some message has been received.
    Ack = 0,
    /// Client sends this message to the server to send a 
    /// message to another client.
    SendMessage = 1,
    /// The client sends this message to the server to inform
    /// that it wants to connect to another client.
    ClientConnectRequest = 2,
    /// The client sends this message to the server to inform 
    /// how to handle the chat request.
    ClientConnectResponse = 3,


    /// List chats
    ListChats = 4,
    /// The client sends this message to the server to 
    /// operate on client's chat list, e.g. add, remove,
    /// or modify chats.
    ModifyChats = 5,
    /// Poll messages
    PollMessages = 6,
    /// Login request
    LoginRequest = 7,
    /// Register request
    RegisterRequest = 8,
    /// Show invites
    ShowInvitesRequest = 9,
    /// Show metadata for a user
    ShowMetadataRequest = 10,

    /// Unknown message
    Unknown = -1,
}

impl Serialize for ClientInstruct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_i32(self.clone() as i32)
    }
}

impl<'de> Deserialize<'de> for ClientInstruct {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let instruct = i32::deserialize(deserializer)?;
        match instruct {
            0 => Ok(ClientInstruct::Ack),
            1 => Ok(ClientInstruct::SendMessage),
            2 => Ok(ClientInstruct::ClientConnectRequest),
            3 => Ok(ClientInstruct::ClientConnectResponse),
            4 => Ok(ClientInstruct::ListChats),
            5 => Ok(ClientInstruct::ModifyChats),
            6 => Ok(ClientInstruct::PollMessages),
            7 => Ok(ClientInstruct::LoginRequest),
            8 => Ok(ClientInstruct::RegisterRequest),
            9 => Ok(ClientInstruct::ShowInvitesRequest),
            10 => Ok(ClientInstruct::ShowMetadataRequest),
            _ => Ok(ClientInstruct::Unknown),
        }
    }
}

