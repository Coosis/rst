use std::fmt::Display;
use serde::{Deserialize, Serialize};
use crate::Uuid;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct UserMetadata {
    // fields like username, avatar, etc
    pub uid: Uuid,
    pub username: String,
    pub avatar: Vec<u8>,
}

impl UserMetadata {
    pub fn new(
        uid: Uuid,
        username: String,
        avatar: Vec<u8>,
        ) -> Self {
        UserMetadata {
            uid,
            username,
            avatar,
        }
    }
}

impl Display for UserMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PublicUserdata: {{ uid: {:?}, username: {} }}",
            self.uid,
            self.username)
    }
}
