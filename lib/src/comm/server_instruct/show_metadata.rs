use rst_proc_macro::TryIntoServerMessage;
use serde::{Deserialize, Serialize};

use crate::user::PublicUserCredential;

#[derive(TryIntoServerMessage)]
#[derive(Serialize, Deserialize)]
/// TODO
pub struct ShowMetadataResponse {
    pub data: Vec<PublicUserCredential>,
}

impl ShowMetadataResponse {
    pub fn new(data: Vec<PublicUserCredential>) -> Self {
        ShowMetadataResponse { data }
    }
}
