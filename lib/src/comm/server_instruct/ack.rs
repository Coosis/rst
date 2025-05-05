use rst_proc_macro::TryIntoServerMessage;
use serde::{Deserialize, Serialize};

#[derive(TryIntoServerMessage)]
#[derive(Serialize, Deserialize)]
/// TODO
pub struct Ack {
}
