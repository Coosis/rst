use tracing::error;

use super::HandleError;
type Result<T> = std::result::Result<T, HandleError>;

pub async fn handle_ack() -> Result<()> {
    error!("Acknowledge instruction not implemented");
    Err(HandleError::HandleError("Acknowledge instruction not implemented".to_string()))
}
