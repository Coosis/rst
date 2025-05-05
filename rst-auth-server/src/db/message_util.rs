use axum::extract::ws::Message;
use lib::comm::ServerMessage;
use crate::AuthError;

pub trait MessageExt {
    fn try_into_ws_msg(
        self,
    ) -> Result<axum::extract::ws::Message, HandleError>;
}

impl<T> MessageExt for T
where T: TryInto<ServerMessage, Error = serde_json::Error>
{
    fn try_into_ws_msg(
        self,
    ) -> Result<axum::extract::ws::Message, HandleError> {
        let msg: ServerMessage = match self.try_into() {
            Ok(msg) => msg,
            Err(e) => return Err(
                HandleError::SerdeJsonError(e)
            )
        };
        
        let json_bytes: Vec<u8> = match msg.try_into() {
            Ok(bytes) => bytes,
            Err(e) => return Err(
                HandleError::SerdeJsonError(e)
            )
        };
        let msg = Message::binary(json_bytes);
        Ok(msg)
    }
}
