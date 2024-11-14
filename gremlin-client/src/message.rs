use serde::{Deserialize as SerdeDeserialize, Deserializer};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::GValue;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestIdV2 {
    #[serde(rename = "@type")]
    pub id_type: String,

    #[serde(rename = "@value")]
    pub value: Uuid,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Message<T> {
    #[serde(rename_all = "camelCase")]
    V1 {
        request_id: Uuid,
        op: String,
        processor: String,
        args: T,
    },
    #[serde(rename_all = "camelCase")]
    V2 {
        request_id: RequestIdV2,
        op: String,
        processor: String,
        args: T,
    },
    #[serde(rename_all = "camelCase")]
    V3 {
        request_id: Uuid,
        op: String,
        processor: String,
        args: T,
    },
}

impl<T> Message<T> {
    #[allow(dead_code)]
    pub fn id(&self) -> &Uuid {
        match self {
            Message::V1 { request_id, .. } => request_id,
            Message::V2 { request_id, .. } => &request_id.value,
            Message::V3 { request_id, .. } => request_id,
        }
    }
}
#[derive(Debug)]
pub struct Response {
    pub request_id: Option<Uuid>,
    pub result: ResponseResult,
    pub status: ReponseStatus,
}

#[derive(Debug)]
pub struct ResponseResult {
    pub data: Option<GValue>,
}

#[derive(Debug, Deserialize)]
pub struct ReponseStatus {
    pub code: i16,
    //Sometimes the message is omitted, default to empty string rather than panic
    #[serde(default, deserialize_with = "map_null_to_default")]
    pub message: String,
}

fn map_null_to_default<'de, D, T>(de: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + SerdeDeserialize<'de>,
{
    Option::<T>::deserialize(de).map(Option::unwrap_or_default)
}

#[cfg(test)]
mod tests {
    use crate::message::ReponseStatus;

    #[test]
    fn handle_no_response_status_message() {
        let parsed: ReponseStatus =
            serde_json::from_str(r#"{"code": 123}"#).expect("Failed to parse test message");
        assert_eq!(123, parsed.code);
        assert_eq!("", parsed.message);
    }

    #[test]
    fn handle_null_response_status_message() {
        let parsed: ReponseStatus = serde_json::from_str(r#"{"code": 123, "message": null}"#)
            .expect("Failed to parse test message");
        assert_eq!(123, parsed.code);
        assert_eq!("", parsed.message);
    }

    #[test]
    fn handle_response_status_message() {
        let parsed: ReponseStatus =
            serde_json::from_str(r#"{"code": 123, "message": "Hello World"}"#)
                .expect("Failed to parse test message");
        assert_eq!(123, parsed.code);
        assert_eq!("Hello World", parsed.message);
    }
}
