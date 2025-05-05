use mongodb::options::UpdateModifications;
use mongodb::results::{InsertOneResult, UpdateResult};
use mongodb::Client;
use crate::HandleError;
use crate::db::{DB, TB_USERS, TB_MESSAGES, TB_CHATS, TB_INVITES};

pub async fn update_one<T>(
    db_client: &Client,
    collection: &str,
    query: bson::Document,
    update: impl Into<UpdateModifications>,
) -> Result<UpdateResult, HandleError> 
where
    T: serde::de::DeserializeOwned + Send + Sync,
{
    db_client.database(DB)
        .collection::<T>(collection)
        .update_one(query, update)
        .await
        .map_err(|e| HandleError::DbError(e))
}

/// Inserts multiple documents into the specified collection.
///
/// # Arguments
/// * `db_client` - A reference to the MongoDB client.
/// * `collection` - The name of the collection to insert into.
/// * `filter` - The filter to use for the insert operation.
pub async fn insert_many<T>(
    db_client: &Client,
    collection: &str,
    filter: bson::Document,
) -> Result<mongodb::Cursor<T>, HandleError> 
where
    T: serde::de::DeserializeOwned + Send + Sync,
{
    db_client.database(DB)
        .collection(collection)
        .find(filter)
        .await
        .map_err(|e| HandleError::DbError(e))
}
