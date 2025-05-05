use super::ServerInstruct;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ServerMessage {
    pub instruct: ServerInstruct,
    pub content: Vec<u8>,
}

impl TryInto<Vec<u8>> for ServerMessage {
    type Error = serde_json::Error;
    fn try_into(self) -> std::result::Result<Vec<u8>, Self::Error> {
        serde_json::to_vec(&self)
    }
}
