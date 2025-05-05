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

use crate::user::PublicUserCredential;

#[derive(Debug)]
#[derive(TryIntoClientMessage)]
#[derive(Serialize, Deserialize)]
/// A request made by a client to another client
/// in hopes to start a chat with that client, 
pub struct ClientConnectRequest {
    /// fields to identify the receiver
    pub to: Vec<PublicUserCredential>,

    /// name for the chat that's about to be made
    pub name: String,
    /// description for the chat that's about to be made
    pub description: String,
}

impl ClientConnectRequest {
    pub fn new(
        to: Vec<PublicUserCredential>,
        name: String,
        description: String,
        ) -> Self {
        ClientConnectRequest {
            to,
            name,
            description,
        }
    }
}
