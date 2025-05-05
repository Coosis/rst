use lib::util::{BsonDocExt, AsyncMongoCursorExt};
use mongodb::Cursor;
use futures::stream::StreamExt;

use crate::AuthError;

impl<T> BsonDocExt<AuthError, T> for Result<Option<bson::Document>, mongodb::error::Error> 
where
    T: serde::de::DeserializeOwned + Send + Sync,
{
    fn decode(
        self,
        custom_error: crate::AuthError
    ) -> Result<T, crate::AuthError> 
        where T: serde::de::DeserializeOwned + Send + Sync
    {
        match self {
            Ok(Some(doc)) => {
                match bson::from_bson(bson::Bson::Document(doc)) {
                    Ok(t) => Ok(t),
                    Err(e) => return Err(
                        crate::AuthError::BsonDeError(e)
                    )
                }
            },
            Ok(None) => Err(custom_error),
            Err(e) => Err(crate::AuthError::DbError(e))
        }
    }
}

impl<T> AsyncMongoCursorExt<crate::AuthError, T> for Result<Cursor<T>, mongodb::error::Error>
where
    T: serde::de::DeserializeOwned + Send + Sync + Unpin,
{
    async fn decode_async(self) -> Result<Vec<T>, AuthError> {
        let mut cursor = self.map_err(AuthError::DbError)?;
        let mut data = Vec::new();

        while let Some(result) = cursor.next().await {
            let item = result.map_err(AuthError::DbError)?;
            data.push(item);
        }

        Ok(data)
    }
}
