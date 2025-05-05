use rst_proc_macro::TryIntoClientMessage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub enum LoginType {
    ByEmail,
    ByPhone,
    ByUserID
}

#[derive(TryIntoClientMessage)]
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct LoginRequest {
    pub login_type: LoginType,
    pub identifier: String,
    pub passwd: String,
}

impl LoginRequest {
    pub fn new(
        login_type: LoginType,
        identifier: String,
        passwd: String,
        ) -> Self {
        LoginRequest {
            login_type,
            identifier,
            passwd,
        }
    }
}
