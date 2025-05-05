use lib::comm::client_instruct::ShowMetadataRequest;
use lib::comm::server_instruct::ShowMetadataResponse;
use lib::user::{MetadataQuery, PublicUserCredential, UserCredential};
use tracing::debug;

use crate::message_util::MessageExt;
use crate::state::AppState;
use crate::{HandleError, MsgChan, Result, AUTH_SERVER};
// type Result<T> = std::result::Result<T, HandleError>;

pub async fn show_metadata(
    state: tokio::sync::MutexGuard<'_, AppState>,
    tx: &MsgChan,
    req: ShowMetadataRequest,
) -> Result<()> {
    debug!("Handling show_metadata");
    // TODO: add other fields
    let mut data = vec![];
    for uid in req.uids {
        let query = MetadataQuery::new(uid);
        let addr = format!(
            "{}/get_metadata",
            AUTH_SERVER,
        );
        let response = state.client.get(&addr)
            .json(&query)
            .send()
            .await?;
        debug!("Response status code: {}", response.status());
        data.push(
            response.json::<PublicUserCredential>()
                .await?
        );
    }
    let msg = ShowMetadataResponse::new(data).try_into_ws_msg()?;
    match tx.send(msg) {
        Ok(_) => {
            tracing::debug!("Sent show_metadata response");
        },
        Err(e) => {
            tracing::error!("Failed to send show_metadata response: {}", e);
            return Err(HandleError::SendError(e.to_string()));
        }
    }
    debug!("Handled show_metadata");
    Ok(())
}
