use argon2::{
    PasswordHasher,
    password_hash::{
        rand_core::OsRng, SaltString
    },
    Argon2
};
use lib::Uuid;
use lib::user::User;
use lib::comm::client_instruct::RegisterRequest;
use mongodb::Client;
use tracing::debug;

use crate::{insert_one, TB_USERS};
use crate::handlers::HandleError;

type Result<T> = std::result::Result<T, HandleError>;

pub async fn register_handler(
    db_client: Client,
    argon: &Argon2<'_>,
    request: RegisterRequest,
    ) -> Result<()> {
    debug!("Handling register request");
    let salt = SaltString::generate(&mut OsRng);
    let passwd_hash = argon
        .hash_password(request.passwd.as_bytes(), &salt)
        .map_err(|_| HandleError::HashError("Failed to hash password".to_string()))?
        .to_string();

    let id = Uuid::now_v7();
    let epoch = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    let user = User::with_register_request(
        request,
        id,
        passwd_hash,
        epoch
        );

    debug!("Handling register request done");
    insert_one::<User>(
        &db_client,
        TB_USERS,
        user).await?;

    Ok(())
}
