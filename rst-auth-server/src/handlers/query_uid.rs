use axum::{extract::State, Json};
use bson::doc;
use lib::Uuid;
use lib::user::{UidQuery, UserCredential};

use crate::{find_one, SharedState, Result, TB_USERS};

pub async fn get_uid(
    State(state): State<SharedState>,
    Json(query): Json<UidQuery>,
) -> Result<Json<Uuid>> {
    if query.email.is_none() && query.phone.is_none() {
        return Err(crate::AuthError::HandleError(
            "Provider either email or phone".to_string(),
        ));
    }

    let mut filter = doc! {};
    if let Some(email) = query.email {
        filter.insert("email", email);
    }
    if let Some(phone) = query.phone {
        filter.insert("phone", phone);
    }

    Ok(Json(find_one::<UserCredential>(
        &state.db_client,
        TB_USERS,
        filter,
        crate::AuthError::NoUserFound("user not found".to_string()),
    ).await?.id))
}
