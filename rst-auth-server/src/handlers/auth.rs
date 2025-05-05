use argon2::{PasswordHash, PasswordVerifier};
use axum::extract::State;
use axum::{self, Json};
use bson::doc;
use jsonwebtoken::{encode, EncodingKey, Header};
use std::time::SystemTime;
use std::ops::Add;
use tracing::info;

use lib::jwt::Jwt;
use lib::user::UserCredential;
use lib::comm::client_instruct::LoginRequest;

use crate::error::AuthError;
use crate::handlers::EXPIRATION_TIME;
use crate::Result;
use crate::{find_one, SharedState, TB_USERS};
use crate::SECRET;

pub async fn auth(
    State(state): State<SharedState>,
    Json(payload): Json<LoginRequest>,
) -> Result<String> {
    info!("Received LoginRequest: {:?}", payload);
    info!("identifier: {:?}", payload.identifier);
    let filter = doc!{ 
        "$or": [
            { "email": payload.identifier.clone() },
            { "phone": payload.identifier.clone() },
            { "id": payload.identifier.clone() }
        ],
    };
    let db_client = state.db_client.clone();
    let user: UserCredential = find_one(
        &db_client,
        TB_USERS, 
        filter, 
        AuthError::NoUserFound("user not found".to_string())
        ).await?;
    info!("User found: {:?}", user.passwd_hash);

    let hash = PasswordHash::new(&user.passwd_hash)
        .map_err(|_| AuthError::HashError("Failed to parse password hash".to_string()))?;

    let argon = state.argon.clone();
    argon.verify_password(
        payload.passwd.as_bytes(),
        &hash)
        .map_err(|_| {
            AuthError::InvalidCredentials("Invalid password".to_string())
        })?;

    let uid = user.id;
    let exp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .add(std::time::Duration::new(EXPIRATION_TIME, 0))
        .as_secs() as i64;
    let token: String = encode(
        &Header::new(jsonwebtoken::Algorithm::HS256), 
        &Jwt {
            uid,
            exp,
        },
        &EncodingKey::from_secret(SECRET))?;
    info!("Login successful for user: {:?}", uid);
    Ok(token)
}
