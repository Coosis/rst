use axum::{extract::State, Json};
use bson::doc;
use lib::user::PublicUserCredential;
use lib::user::MetadataQuery;
use lib::user::UserCredential;

use crate::{find_one, SharedState, TB_USERS};
use crate::Result;

pub async fn get_credentials(
    State(state): State<SharedState>,
    Json(query): Json<MetadataQuery>,
) -> Result<Json<PublicUserCredential>> {
    let filter = doc! {
        "id": query.uid
    };

    let result: UserCredential = find_one(
        &state.db_client,
        TB_USERS,
        filter,
        crate::AuthError::NoUserFound("user not found".to_string()),
    ).await?;

    Ok(Json(result.to_public()))
}
