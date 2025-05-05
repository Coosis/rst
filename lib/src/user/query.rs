use serde::{Deserialize, Serialize};

use crate::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct UidQuery {
    pub email: Option<String>,
    pub phone: Option<String>,
}

impl UidQuery {
    pub fn new(email: Option<String>, phone: Option<String>) -> Self {
        UidQuery { email, phone }
    }
}

#[derive(Deserialize, Serialize)]
pub struct MetadataQuery {
    pub uid: Uuid
}

impl MetadataQuery {
    pub fn new(uid: Uuid) -> Self {
        MetadataQuery { uid }
    }
}
