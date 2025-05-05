pub trait BsonDocExt<E, T> 
where
    T: serde::de::DeserializeOwned + Send + Sync,
{
    // type Item;
    fn decode(
        self,
        custom_error: E
    ) -> Result<T, E>;
        // where Item: serde::de::DeserializeOwned + Send + Sync;
}

pub trait AsyncMongoCursorExt<E, T> {
    async fn decode_async(self) -> Result<Vec<T>, E>
    where 
        T: serde::de::DeserializeOwned + Send + Sync;
}
