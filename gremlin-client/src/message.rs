use serde_derive::{Deserialize, Serialize};
use serde_json::Map;
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

#[derive(Serialize)]
pub struct GremlinScript {
    gremlin: String,
    bindings: Map<String, Value>,
    aliases: Map<String, Value>,
    language: String,
}

pub fn gremlin(
    script: String,
    bindings: Map<String, Value>,
    alias: Option<String>,
) -> Message<GremlinScript> {
    let aliases = alias
        .map(|s| {
            let mut map = Map::new();
            map.insert(String::from("g"), Value::String(s));
            map
        })
        .unwrap_or_else(Map::new);
    Message {
        request_id: Uuid::new_v4(),
        op: String::from("eval"),
        processor: String::from(""),
        args: GremlinScript {
            gremlin: script,
            bindings,
            aliases,
            language: String::from("gremlin-groovy"),
        },
    }
}
