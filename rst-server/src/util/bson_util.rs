use lib::util::{BsonDocExt, AsyncMongoCursorExt};
use mongodb::Cursor;
use futures::stream::StreamExt;

use crate::HandleError;

impl<T> BsonDocExt<HandleError, T> for Result<Option<bson::Document>, mongodb::error::Error> 
where T: serde::de::DeserializeOwned + Send + Sync
{
    fn decode(
        self,
        custom_error: HandleError
    ) -> Result<T, HandleError> 
    {
        match self {
            Ok(Some(doc)) => {
                match bson::from_bson(bson::Bson::Document(doc)) {
                    Ok(t) => Ok(t),
                    Err(e) => return Err(
                        HandleError::BsonDeError(e)
                    )
                }
            },
            Ok(None) => Err(custom_error),
            Err(e) => Err(HandleError::DbError(e))
        }
    }
}

impl<T> AsyncMongoCursorExt<HandleError, T> for Result<Cursor<T>, mongodb::error::Error>
where
    T: serde::de::DeserializeOwned + Send + Sync + Unpin,
{
    async fn decode_async(self) -> Result<Vec<T>, HandleError> {
        let mut cursor = self.map_err(HandleError::DbError)?;
        let mut data = Vec::new();

        while let Some(result) = cursor.next().await {
            let item = result.map_err(HandleError::DbError)?;
            data.push(item);
        }

        Ok(data)
    }
}
