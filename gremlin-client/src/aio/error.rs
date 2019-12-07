use crate::GremlinError;
use async_tungstenite::tungstenite;

impl From<tungstenite::error::Error> for GremlinError {
    fn from(e: tungstenite::error::Error) -> GremlinError {
        GremlinError::WebSocketAsync(e)
    }
}
