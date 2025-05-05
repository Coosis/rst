use rst_proc_macro::TryIntoServerMessage;
use serde::{Deserialize, Serialize};

use crate::message::Message;

#[derive(TryIntoServerMessage)]
#[derive(Serialize, Deserialize)]
pub struct PushMessage {
    pub message: Message,
}

impl PushMessage {
    pub fn new(message: Message) -> Self {
        PushMessage { message }
    }
}
