use crate::structure::GValue;

use thiserror::Error;

#[cfg(feature = "async_gremlin")]
use mobc;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Error)]
pub enum GremlinError {
    #[error("data store disconnected")]
    Generic(String),

    #[error(transparent)]
    WebSocket(#[from] tungstenite::error::Error),

    #[error(transparent)]
    Pool(#[from] r2d2::Error),

    #[error("Got wrong type {0:?}")]
    WrongType(GValue),

    #[error("Cast error: {0}")]
    Cast(String),

    #[error("JSON error: {0}")]
    Json(String),

    #[error("Request error: {0:?} ")]
    Request((i16, String)),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),

    #[cfg(feature = "async_gremlin")]
    #[error(transparent)]
    WebSocketAsync(#[from] async_tungstenite::tungstenite::Error),
    #[cfg(feature = "async_gremlin")]
    #[error(transparent)]
    ChannelSend(#[from] futures::channel::mpsc::SendError),
    #[error(transparent)]
    Uuid(#[from] uuid::Error),
}

#[cfg(feature = "async_gremlin")]
impl From<mobc::Error<GremlinError>> for GremlinError {
    fn from(e: mobc::Error<GremlinError>) -> GremlinError {
        match e {
            mobc::Error::Inner(e) => e,
            mobc::Error::BadConn => {
                GremlinError::Generic(String::from("Async pool bad connection"))
            }
            mobc::Error::Timeout => GremlinError::Generic(String::from("Async pool timeout")),
        }
    }
}
