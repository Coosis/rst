use std::hash::Hash;

use axum::extract::ws::Message;
use lib::jwt::Jwt;

#[derive(Clone, Debug)]
pub struct MsgChan {
    pub jwt: Jwt,
    pub chan: tokio::sync::mpsc::UnboundedSender<Message>,
}

impl MsgChan {
    pub fn new(
        jwt: Jwt,
        chan: tokio::sync::mpsc::UnboundedSender<Message>,
    ) -> Self {
        MsgChan { jwt, chan }
    }
}

impl std::cmp::Eq for MsgChan { }

impl std::cmp::PartialEq for MsgChan {
    fn eq(&self, other: &Self) -> bool {
        self.jwt == other.jwt
    }
}

impl Hash for MsgChan {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.jwt.hash(state);
    }
}
