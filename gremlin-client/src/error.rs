use crate::structure::GValue;
use std::error::Error;
use std::fmt::Display;
use websocket::WebSocketError;
use uuid::parser::ParseError;

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
    Uuid(ParseError),
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
