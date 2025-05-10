use lib::comm::client_instruct::ShowMetadataRequest;
use lib::comm::server_instruct::ShowMetadataResponse;
use lib::jwt::Jwt;
use lib::user::{MetadataQuery, PublicUserCredential};
use tracing::debug;

use crate::message_util::MessageExt;
use crate::{LockedState, Result, AUTH_SERVER};

pub async fn show_metadata(
    token: Jwt,
    state: &LockedState<'_>,
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
    match state.broadcast(token.uid, msg) {
        Ok(_) => {
            tracing::debug!("Sent show_metadata response");
        },
        Err(e) => {
            tracing::error!("Failed to send show_metadata response: {}", e);
            return Err(e);
        }
    }
    debug!("Handled show_metadata");
    Ok(())
}
