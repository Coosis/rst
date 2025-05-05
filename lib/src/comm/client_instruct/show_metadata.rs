use rst_proc_macro::TryIntoClientMessage;
use serde::{Deserialize, Serialize};

use crate::Uuid;

#[derive(TryIntoClientMessage)]
#[derive(Serialize, Deserialize)]
pub struct ShowMetadataRequest {
    pub token: String,
    pub uids: Vec<Uuid>,
}

impl ShowMetadataRequest {
    pub fn new(
        token: String,
        uids: Vec<Uuid>
    ) -> Self {
        ShowMetadataRequest {
            token,
            uids,
        }
    }
}
