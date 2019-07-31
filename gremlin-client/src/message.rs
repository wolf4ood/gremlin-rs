use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Message<T> {
    pub request_id: Uuid,
    op: String,
    processor: String,
    args: T,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub request_id: Uuid,
    pub result: ResponseResult,
    pub status: ReponseStatus,
}

#[derive(Debug, Deserialize)]
pub struct ResponseResult {
    pub data: Value,
}

#[derive(Debug, Deserialize)]
pub struct ReponseStatus {
    pub code: i16,
    pub message: String,
}

pub fn message_with_args<T>(op: String, processor: String, args: T) -> Message<T> {
    message_with_args_and_uuid(op, processor, Uuid::new_v4(), args)
}

pub fn message_with_args_and_uuid<T>(
    op: String,
    processor: String,
    id: Uuid,
    args: T,
) -> Message<T> {
    Message {
        request_id: id,
        op,
        processor,
        args,
    }
}
