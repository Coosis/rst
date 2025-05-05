/* chat creating diagram
 *
 * 1. client1 ------- ClientConnectRequest  -------> server
 * 2. server  ------- ServerConnectRequest  -------> client2
 * 3. server  <------ ClientConnectResponse -------- client2
 * 4. client1 <------ ServerConnectResponse -------- server
 * 4. server  ------- ServerConnectResponse -------> client2
 */

use rst_proc_macro::TryIntoServerMessage;
use serde::{Deserialize, Serialize};

use crate::Uuid;

#[derive(Debug)]
#[derive(TryIntoServerMessage)]
#[derive(Serialize, Deserialize)]
/// A request made by a client to another client
/// in hopes to start a chat with that client, 
pub struct ServerConnectRequest {
    /// uid from the sender
    pub from: Uuid,
    /// timestamp of the request, 
    /// the moment the server processes the request
    pub timestamp: i64,

    /// name for the chat that's about to be made
    pub name: String,
    /// description for the chat that's about to be made
    pub description: String,
}

impl ServerConnectRequest {
    pub fn new(
        from: Uuid,
        timestamp: i64,
        name: String,
        description: String,
        ) -> Self {
        ServerConnectRequest {
            from,
            timestamp,
            name,
            description,
        }
    }
}
