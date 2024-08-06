use crate::GremlinError;
use async_tungstenite::tungstenite;

impl From<&tungstenite::error::Error> for GremlinError {
    fn from(e: &tungstenite::error::Error) -> GremlinError {
        //Some of the tungstenite errors are cloneable or can be lightly recreated
        //for those that cannot be, their message is wrapped in a GremlinError::Generic
        //this does change the observed error type. In the future maybe sending tungstenite errors
        //wrapped in an Arc can avoid this
        let error = match e {
            tungstenite::error::Error::AlreadyClosed => tungstenite::error::Error::AlreadyClosed,
            tungstenite::error::Error::ConnectionClosed => {
                tungstenite::error::Error::ConnectionClosed
            }
            tungstenite::Error::Protocol(e) => tungstenite::Error::Protocol(e.clone()),
            tungstenite::Error::Utf8 => tungstenite::Error::Utf8,
            tungstenite::Error::AttackAttempt => tungstenite::Error::AttackAttempt,
            _ => return GremlinError::Generic(format!("Error from ws {}", e)),
        };
        GremlinError::WebSocketAsync(error)
    }
}
