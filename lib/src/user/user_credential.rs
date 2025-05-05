use std::fmt::Display;
use serde::{Deserialize, Serialize};
use crate::{comm::client_instruct::RegisterRequest, Uuid};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct UserCredential {
    // email, phone, uid are usually obtained from auth service
    pub email: Option<String>,
    pub phone: Option<String>,
    pub id: Uuid,
    pub passwd_hash: String,
    pub created_at: u64,
    // other fields like username, avatar, etc. are obtained from somewhere else
}

impl UserCredential {
    pub fn new(
        email: Option<String>,
        phone: Option<String>,
        id: Uuid,
        passwd_hash: String,
        created_at: u64,
        ) -> Self {
        UserCredential {
            email,
            phone,
            id,
            passwd_hash,
            created_at,
        }
    }

    pub fn to_public(self) -> PublicUserCredential {
        PublicUserCredential::from_user_credential(self)
    }
}

impl Display for UserCredential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PublicUserdata: {{ email: {:?}, phone: {:?}, uid: {:?} }}",
            self.email,
            self.phone,
            self.id,
        )
    }
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct PublicUserCredential {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub id: Option<Uuid>,
    pub created_at: Option<u64>,
}

impl Display for PublicUserCredential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let created_at = match self.created_at {
            Some(at) => at.to_string(),
            None => "None".to_string(),
        };
        write!(f, "PublicUserdata: {{ email: {:?}, phone: {:?}, uid: {:?}, \
            created_at: {} }}",
            self.email,
            self.phone,
            self.id,
            created_at,
        )
    }
}

impl PublicUserCredential {
    pub fn new(
        email: Option<String>,
        phone: Option<String>,
        id: Option<Uuid>,
        created_at: Option<u64>,
        ) -> Self {
        PublicUserCredential {
            email,
            phone,
            id,
            created_at,
        }
    }
    
    pub fn from_user_credential(
        user_credential: UserCredential,
    ) -> Self {
        PublicUserCredential {
            email: user_credential.email,
            phone: user_credential.phone,
            id: Some(user_credential.id),
            created_at: Some(user_credential.created_at),
        }
    }
}
