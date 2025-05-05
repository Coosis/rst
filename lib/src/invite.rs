use serde::{Serialize, Deserialize};
use crate::Uuid;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
/// An invitation to join a chat room
pub struct Invitation {
    pub id: Uuid,
    pub from: Uuid,
    pub to: Uuid,
    pub chat_id: Uuid,
}

impl Invitation {
    pub fn new(
        from: Uuid,
        to: Uuid,
        chat_id: Uuid
    ) -> Self {
        Invitation {
            id: Uuid::new_v4(),
            from,
            to,
            chat_id,
        }
    }
}
