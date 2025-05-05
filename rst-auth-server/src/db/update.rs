use mongodb::options::UpdateModifications;
use mongodb::results::{InsertOneResult, UpdateResult};
use mongodb::Client;
use crate::AuthError;
use crate::DB;

pub async fn update_one<T>(
    db_client: &Client,
    collection: &str,
    query: bson::Document,
    update: impl Into<UpdateModifications>,
) -> Result<UpdateResult, AuthError> 
where
    T: serde::de::DeserializeOwned + Send + Sync,
{
    db_client.database(DB)
        .collection::<T>(collection)
        .update_one(query, update)
        .await
        .map_err(|e| AuthError::DbError(e))
}
