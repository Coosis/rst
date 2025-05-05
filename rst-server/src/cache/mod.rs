use lib::Uuid;
use redis::AsyncCommands;
use tracing::debug;
use std::ops::DerefMut;
use crate::{Result, HandleError};

pub async fn get_uuid(
    multiplexed: &mut redis::aio::MultiplexedConnection,
    token: String
) -> Result<Uuid> {
    Ok(
        multiplexed.get::<String, String>(token)
        .await
        .map_err(|_| {
            HandleError::InvalidCredentials("Invalid token".to_string())
        })
        .and_then(|uid: String| {
            Uuid::parse_str(&uid)
                .map_err(|_| {
                    HandleError::InvalidCredentials("Invalid token".to_string())
                })
        })?
    )
}
