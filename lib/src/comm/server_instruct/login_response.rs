use rst_proc_macro::TryIntoServerMessage;
use serde::{Deserialize, Serialize};

use crate::Uuid;

/// A response sent by server to client, as a 
/// response to a login request made by client
#[derive(Debug)]
#[derive(TryIntoServerMessage)]
#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    // Session token returned by auth server
    // pub token: String,
}

impl LoginResponse {
    pub fn new() -> Self {
        LoginResponse {
            // token,
        }
    }
}
