use rst_proc_macro::TryIntoClientMessage;
use serde::{Deserialize, Serialize};

// use crate::Uuid;

#[derive(TryIntoClientMessage)]
#[derive(Serialize, Deserialize)]
pub struct ShowInvitesRequest {
    // pub token: Uuid,
}

impl ShowInvitesRequest {
    pub fn new() -> Self {
        ShowInvitesRequest {
            // token,
        }
    }
}
