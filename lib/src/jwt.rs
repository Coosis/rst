use crate::Uuid;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{errors::Result, EncodingKey};

#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
pub struct Jwt {
    pub uid: Uuid,
    pub exp: i64,
}

impl Jwt {
    pub fn encode(self, key: &EncodingKey) -> Result<String> {
        let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
        jsonwebtoken::encode(&header, &self, key)
    }
}
