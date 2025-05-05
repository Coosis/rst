use super::ClientInstruct;
use serde::{Deserialize, Serialize};
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ClientMessage {
    pub instruct: ClientInstruct,
    pub content: Vec<u8>,
}

impl TryInto<Vec<u8>> for ClientMessage {
    type Error = serde_json::Error;
    fn try_into(self) -> std::result::Result<Vec<u8>, Self::Error> {
        serde_json::to_vec(&self)
    }
}
