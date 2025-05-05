use rst_proc_macro::TryIntoClientMessage;
use serde::{Deserialize, Serialize};

use crate::{message::MessageBuilder, Uuid};

#[derive(Clone)]
#[derive(TryIntoClientMessage)]
#[derive(Serialize, Deserialize)]
pub struct SendMessage {
    pub message: MessageBuilder,
}

impl SendMessage {
    pub fn new(
        message: MessageBuilder,
    ) -> Self {
        SendMessage {
            message,
        }
    }
}
