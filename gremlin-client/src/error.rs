use crate::structure::GValue;
use std::error::Error;
use std::fmt::Display;
use uuid::parser::ParseError;
use websocket::WebSocketError;

#[cfg(feature = "async_std")]
use async_tungstenite::tungstenite;
#[cfg(feature = "async_gremlin")]
use mobc;

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum GremlinError {
    Generic(String),
    WebSocket(WebSocketError),
    Pool(r2d2::Error),
    WrontType(GValue),
    Cast(String),
    Json(String),
    Request((i16, String)),
    Serde(serde_json::Error),
    #[cfg(feature = "async_gremlin")]
    WebSocketAsync(tungstenite::error::Error),
    Uuid(ParseError),
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

impl From<WebSocketError> for GremlinError {
    fn from(e: WebSocketError) -> GremlinError {
        GremlinError::WebSocket(e)
    }
}

impl From<r2d2::Error> for GremlinError {
    fn from(e: r2d2::Error) -> GremlinError {
        GremlinError::Pool(e)
    }
}

impl From<serde_json::Error> for GremlinError {
    fn from(e: serde_json::Error) -> GremlinError {
        GremlinError::Serde(e)
    }
}
impl From<ParseError> for GremlinError {
    fn from(e: ParseError) -> GremlinError {
        GremlinError::Uuid(e)
    }
}
impl Display for GremlinError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{:?}", self)
    }
}
impl Error for GremlinError {}
