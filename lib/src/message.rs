use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

use crate::Uuid;
use crate::content::Content;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
/// A chat message
pub struct Message {
    pub id: Uuid,
    pub from: Uuid,
    /// the chat id
    pub to: Uuid,
    pub contents: Vec<Content>,
    pub timestamp: i64,
}

impl<'a> Message {
    pub fn new(
        from: Uuid,
        to: Uuid,
        contents: Vec<Content>,
        timestamp: i64
        ) -> Self {
        Message {
            id: Uuid::new_v4(),
            from,
            to,
            contents,
            timestamp,
        }
    }

    pub fn to_bytes(&self) -> serde_json::Result<Vec<u8>> {
        serde_json::to_vec(self)
    }

    pub fn from_bytes(bytes: &'a [u8]) -> serde_json::Result<Self> {
        serde_json::from_slice(bytes)
    }
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct MessageBuilder {
    pub to: Uuid,
    pub contents: Vec<Content>,
}

impl MessageBuilder {
    pub fn new(to: Uuid) -> Self {
        MessageBuilder {
            to,
            contents: Vec::new(),
        }
    }

    pub fn push_mut(&mut self, contents: Content) -> &mut Self {
        self.contents.push(contents);
        self
    }

    pub fn push(mut self, contents: Content) -> Self {
        self.contents.push(contents);
        self
    }

    pub fn build(
        self,
        from: Uuid,
    ) -> Message {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as i64;
        Message::new(
            from, 
            self.to, 
            self.contents, 
            now
        )
    }
}
