use rst_proc_macro::TryIntoClientMessage;
use serde::{Deserialize, Serialize};

/// client-side user info, used for registering
#[derive(TryIntoClientMessage)]
#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub username: String,
    pub passwd: String,
}

impl RegisterRequest {
    pub fn new(
        email: Option<String>,
        phone: Option<String>,
        username: String,
        passwd: String,
        ) -> Self {
        RegisterRequest {
            email,
            phone,
            username,
            passwd,
        }
    }
}
