use rst_proc_macro::TryIntoServerMessage;
use serde::{Deserialize, Serialize};

use crate::chat::Chat;

#[derive(TryIntoServerMessage)]
#[derive(Serialize, Deserialize)]
/// TODO
pub struct ShowChatsResponse {
    pub chats: Vec<Chat>,
}

impl ShowChatsResponse {
    pub fn new(data: Vec<Chat>) -> Self {
        ShowChatsResponse { chats: data }
    }
}
