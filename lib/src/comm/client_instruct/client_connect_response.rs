/* chat creating diagram
 *
 * 1. client1 ------- ClientConnectRequest  -------> server
 * 2. server  ------- ServerConnectRequest  -------> client2
 * 3. server  <------ ClientConnectResponse -------- client2
 * 4. client1 <------ ServerConnectResponse -------- server
 * 4. server  ------- ServerConnectResponse -------> client2
 */

use serde::{Deserialize, Serialize};
use rst_proc_macro::TryIntoClientMessage;

use crate::Uuid;

#[derive(Debug)]
#[derive(TryIntoClientMessage)]
#[derive(Serialize, Deserialize)]
/// A request made by a client to another client
/// in hopes to start a chat with that client, 
pub struct ClientConnectResponse {
    // /// session token of the client that's making the response
    // pub token: Uuid,

    /// uuid for the invitation
    pub invite_id: Uuid,
}

impl ClientConnectResponse {
    pub fn new(
        // token: Uuid,
        invite_id: Uuid,
        ) -> Self {
        ClientConnectResponse {
            // token,
            invite_id
        }
    }
}
