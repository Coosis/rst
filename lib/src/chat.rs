use serde::{Serialize, Deserialize};
use crate::Uuid;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
/// Chat's metadata
pub struct Chat {
    /// Chat's id
    pub id: Uuid,
    /// Chat's name
    pub name: String,
    /// Chat's description
    pub description: String,
    /// Created time, in seconds since epoch
    pub created: i64,
    /// Last updated, in seconds since epoch
    pub updated: i64,
    /// Chat's members(their id)
    pub members: Vec<Uuid>,
    /// Chat's messages(their id)
    pub messages: Vec<Uuid>,
}

impl Chat {
    pub fn new(
        id: Uuid,
        name: String,
        description: String,
        created: i64,
        updated: i64,
        members: Vec<Uuid>,
        messages: Vec<Uuid>,
        ) -> Self {
        Chat {
            id,
            name,
            description,
            created,
            updated,
            members,
            messages,
        }
    }
}
