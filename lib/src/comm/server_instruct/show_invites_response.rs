use rst_proc_macro::TryIntoServerMessage;
use serde::{Deserialize, Serialize};

use crate::invite::Invitation;

#[derive(Debug)]
#[derive(TryIntoServerMessage)]
#[derive(Serialize, Deserialize)]
pub struct ShowInvitesResponse {
    pub invites: Vec<Invitation>,
}

impl ShowInvitesResponse {
    pub fn new(invites: Vec<Invitation>) -> Self {
        ShowInvitesResponse { invites }
    }
}
