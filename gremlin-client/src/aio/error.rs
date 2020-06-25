use crate::GremlinError;
use async_tungstenite::tungstenite;

impl From<&tungstenite::error::Error> for GremlinError {
    fn from(e: &tungstenite::error::Error) -> GremlinError {
        let error = match e {
            tungstenite::error::Error::AlreadyClosed => tungstenite::error::Error::AlreadyClosed,
            tungstenite::error::Error::ConnectionClosed => {
                tungstenite::error::Error::ConnectionClosed
            }
            _ => return GremlinError::Generic(format!("Error from ws {}", e)),
        };
        GremlinError::WebSocketAsync(error)
    }
}
