use argon2::{
    PasswordHasher,
    password_hash::{
        rand_core::OsRng, SaltString
    },
};
use axum::{extract::State, Json};
use lib::{user::UserCredential, Uuid};
use lib::comm::client_instruct::RegisterRequest;
use tracing::debug;

use crate::{insert_one, SharedState, TB_USERS};
use crate::AuthError;
use crate::Result;

pub async fn register(
    State(state): State<SharedState>,
    Json(request): Json<RegisterRequest>,
    ) -> Result<()> {
    debug!("Handling register request");
    let salt = SaltString::generate(&mut OsRng);
    let argon = &state.argon;
    let passwd = request.passwd.trim_ascii();
    debug!("password: {:?}", passwd);

    let passwd_hash = argon
        .hash_password(passwd.as_bytes(), &salt)
        .map_err(|_| AuthError::HashError("Failed to hash password".to_string()))?
        .to_string();

    let id = Uuid::now_v7();
    let epoch = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    let user = UserCredential::new(
        request.email, 
        request.phone, 
        id,
        passwd_hash,
        epoch
    );
    debug!("Handling register request done");
    let db_client = state.db_client.clone();
    insert_one::<UserCredential>(
        &db_client,
        TB_USERS,
        user).await?;

    Ok(())
}
