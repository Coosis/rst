use lib::util::{BsonDocExt, AsyncMongoCursorExt};
use mongodb::Client;
use crate::AuthError;
use crate::DB;

/// Finds a single document in the specified collection.
///
/// # Arguments
/// * `db_client` - A reference to the MongoDB client.
/// * `collection` - The name of the collection to search.
/// * `filter` - The filter to use for the search operation.
/// * `custom_error` - A custom error to return if anything goes wrong.
pub async fn find_one<T>(
    db_client: &Client,
    collection: &str,
    filter: bson::Document,
    custom_error: AuthError,
) -> Result<T, AuthError> 
where
    T: serde::de::DeserializeOwned + Send + Sync,
{
    Ok(
        db_client.database(DB)
        .collection(collection)
        .find_one(filter)
        .await
        .decode(custom_error)?
    )
}

/// Finds multiple documents in the specified collection.
///
/// # Arguments
/// * `db_client` - A reference to the MongoDB client.
/// * `collection` - The name of the collection to search.
/// * `filter` - The filter to use for the search operation.
pub async fn find_many<T>(
    db_client: &Client,
    collection: &str,
    filter: bson::Document,
) -> Result<Vec<T>, AuthError> 
where
    T: serde::de::DeserializeOwned + Send + Sync + Unpin,
{
    Ok(
        db_client.database(DB)
            .collection(collection)
            .find(filter)
            .await
            .decode_async()
            .await?
    )
}

/// Finds a single document and deletes it from the specified collection.
///
/// # Arguments
/// * `db_client` - A reference to the MongoDB client.
/// * `collection` - The name of the collection to delete from.
/// * `filter` - The filter to use for the delete operation.
pub async fn find_one_and_delete<T>(
    db_client: &Client,
    collection: &str,
    filter: bson::Document,
) -> Result<Option<T>, mongodb::error::Error>
where
    T: serde::de::DeserializeOwned + Send + Sync,
{
    db_client.database(DB)
        .collection(collection)
        .find_one_and_delete(filter)
        .await
}
