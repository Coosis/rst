use std::borrow::Borrow;
use mongodb::results::{InsertManyResult, InsertOneResult};
use mongodb::Client;
use serde::Serialize;

use crate::AuthError;
use crate::db::DB;

/// Inserts a single document into the specified collection.
///
/// # Arguments
/// * `db_client` - A reference to the MongoDB client.
/// * `collection` - The name of the collection to insert into.
/// * `doc` - The document to insert.
pub async fn insert_one<T>(
    db_client: &Client,
    collection: &str,
    doc: impl Borrow<T>,
) -> Result<InsertOneResult, AuthError> 
where
    T: serde::de::DeserializeOwned + Send + Sync + Serialize
{
    db_client.database(DB)
        .collection(collection)
        .insert_one(doc)
        .await
        .map_err(|e| AuthError::DbError(e))
}

/// Inserts multiple documents into the specified collection.
///
/// # Arguments
/// * `db_client` - A reference to the MongoDB client.
/// * `collection` - The name of the collection to insert into.
/// * `doc` - The documents to insert.
pub async fn insert_many<T, U>(
    db_client: &Client,
    collection: &str,
    doc: impl IntoIterator<Item = U>,
) -> Result<InsertManyResult, AuthError> 
where
    T: serde::de::DeserializeOwned + Send + Sync + Serialize,
    U: Borrow<T>
{
    db_client.database(DB)
        .collection(collection)
        .insert_many(doc)
        .await
        .map_err(|e| AuthError::DbError(e))
}
