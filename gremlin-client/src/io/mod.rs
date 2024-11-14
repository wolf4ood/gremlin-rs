#[macro_use]
mod macros;
mod graph_binary_v1;
mod serializer_v2;
mod serializer_v3;

use crate::conversion::ToGValue;
use crate::message::{ReponseStatus, RequestIdV2, Response, ResponseResult};
use crate::process::traversal::{Order, Scope};
use crate::structure::{Cardinality, Direction, GValue, Merge, T};
use graph_binary_v1::GraphBinaryV1Deser;
use serde_derive::Deserialize;
use serde_json::{json, Map, Value};
use std::collections::HashMap;
use std::string::ToString;
use uuid::Uuid;

use crate::{io::graph_binary_v1::GraphBinaryV1Ser, GremlinError, GremlinResult, Message};

#[derive(Debug, Clone, PartialEq)]
pub enum IoProtocol {
    GraphSONV2,
    GraphSONV3,
    GraphBinaryV1,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MiddleResponse {
    pub request_id: Uuid,
    pub result: MiddleResponseResult,
    pub status: ReponseStatus,
}

#[derive(Debug, Deserialize)]
pub struct MiddleResponseResult {
    pub data: Value,
}

impl IoProtocol {
    pub fn read_response(&self, response: Vec<u8>) -> GremlinResult<Response> {
        match self {
            IoProtocol::GraphSONV2 => {
                let middle_form: MiddleResponse =
                    serde_json::from_slice(&response).map_err(GremlinError::from)?;
                Ok(Response {
                    request_id: Some(middle_form.request_id),
                    result: ResponseResult {
                        data: serializer_v2::deserializer_v2(&middle_form.result.data).map(Some)?,
                    },
                    status: middle_form.status,
                })
            }
            IoProtocol::GraphSONV3 => {
                let middle_form: MiddleResponse =
                    serde_json::from_slice(&response).map_err(GremlinError::from)?;
                Ok(Response {
                    request_id: Some(middle_form.request_id),
                    result: ResponseResult {
                        data: serializer_v3::deserializer_v3(&middle_form.result.data).map(Some)?,
                    },
                    status: middle_form.status,
                })
            }
            IoProtocol::GraphBinaryV1 => {
                graph_binary_v1::ResponseMessage::from_be_bytes(&mut response.iter())
                    .map(|middle| middle.into())
            }
        }
    }

    pub fn build_message(
        &self,
        op: &str,
        processor: &str,
        args: HashMap<String, GValue>,
        request_id: Option<Uuid>,
    ) -> GremlinResult<(Uuid, Vec<u8>)> {
        let content_type = self.content_type();
        let request_id = request_id.unwrap_or_else(Uuid::new_v4);
        let message_bytes = match self {
            IoProtocol::GraphSONV2 | IoProtocol::GraphSONV3 => {
                let op = op.into();
                let processor = processor.into();
                let args = self.write_graphson(&GValue::from(args))?;
                let message = match self {
                    IoProtocol::GraphSONV2 => Message::V2 {
                        request_id: RequestIdV2 {
                            id_type: "g:UUID".to_string(),
                            value: request_id,
                        },
                        op,
                        processor,
                        args,
                    },
                    IoProtocol::GraphSONV3 => Message::V3 {
                        request_id,
                        op,
                        processor,
                        args,
                    },
                    _ => unreachable!("Invalid branch"),
                };

                let msg = serde_json::to_string(&message).map_err(GremlinError::from)?;
                let payload = String::from("") + content_type + &msg;
                let mut binary = payload.into_bytes();
                binary.insert(0, content_type.len() as u8);
                binary
            }
            IoProtocol::GraphBinaryV1 => {
                let mut message_bytes: Vec<u8> = Vec::new();
                graph_binary_v1::RequestMessage {
                    request_id,
                    op,
                    processor,
                    args,
                }
                .to_be_bytes(&mut message_bytes)?;
                message_bytes
            }
        };
        Ok((request_id, message_bytes))
    }

    fn write_graphson(&self, value: &GValue) -> GremlinResult<Value> {
        match (self, value) {
            (_, GValue::Double(d)) => Ok(json!({
                "@type" : "g:Double",
                "@value" : d
            })),
            (_, GValue::Float(f)) => Ok(json!({
                "@type" : "g:Float",
                "@value" : f
            })),
            (_, GValue::Int32(i)) => Ok(json!({
                "@type" : "g:Int32",
                "@value" : i
            })),
            (_, GValue::Int64(i)) => Ok(json!({
                "@type" : "g:Int64",
                "@value" : i
            })),
            (_, GValue::String(s)) => Ok(Value::String(s.clone())),
            (_, GValue::Uuid(s)) => Ok(json!({
                "@type" : "g:UUID",
                "@value" : s.to_string()
            })),
            (_, GValue::Date(d)) => Ok(json!({
                "@type" : "g:Date",
                "@value" : d.timestamp_millis()
            })),
            (IoProtocol::GraphSONV2, GValue::List(d)) => {
                let elements: GremlinResult<Vec<Value>> =
                    d.iter().map(|e| self.write_graphson(e)).collect();
                Ok(json!(elements?))
            }
            (IoProtocol::GraphSONV3, GValue::List(d)) => {
                let elements: GremlinResult<Vec<Value>> =
                    d.iter().map(|e| self.write_graphson(e)).collect();
                Ok(json!({
                    "@type" : "g:List",
                    "@value" : elements?
                }))
            }
            (_, GValue::P(p)) => Ok(json!({
                "@type" : "g:P",
                "@value" : {
                    "predicate" : p.operator(),
                    "value" : self.write_graphson(p.value())?
                }
            })),
            (_, GValue::Bytecode(code)) => {
                let steps: GremlinResult<Vec<Value>> = code
                    .steps()
                    .iter()
                    .map(|m| {
                        let mut instruction = vec![];
                        instruction.push(Value::String(m.operator().clone()));

                        let arguments: GremlinResult<Vec<Value>> =
                            m.args().iter().map(|a| self.write_graphson(a)).collect();

                        instruction.extend(arguments?);
                        Ok(Value::Array(instruction))
                    })
                    .collect();

                let sources: GremlinResult<Vec<Value>> = code
                    .sources()
                    .iter()
                    .map(|m| {
                        let mut instruction = vec![];
                        instruction.push(Value::String(m.operator().clone()));

                        let arguments: GremlinResult<Vec<Value>> =
                            m.args().iter().map(|a| self.write_graphson(a)).collect();

                        instruction.extend(arguments?);
                        Ok(Value::Array(instruction))
                    })
                    .collect();
                Ok(json!({
                    "@type" : "g:Bytecode",
                    "@value" : {
                        "step" : steps?,
                        "source" : sources?
                    }
                }))
            }
            (_, GValue::Vertex(v)) => {
                let id = self.write_graphson(&v.id().to_gvalue())?;
                Ok(json!({
                    "@type" : "g:Vertex",
                    "@value" : {
                        "id" :  id,
                    }
                }))
            }
            (IoProtocol::GraphSONV2, GValue::Map(map)) => {
                let mut params = Map::new();

                for (k, v) in map.iter() {
                    params.insert(
                        self.write_graphson(&k.clone().into())?
                            .as_str()
                            .ok_or_else(|| {
                                GremlinError::Generic("Non-string key value.".to_string())
                            })?
                            .to_string(),
                        self.write_graphson(&v)?,
                    );
                }

                Ok(json!(params))
            }
            (IoProtocol::GraphSONV3, GValue::Map(map)) => {
                let mut params = vec![];

                for (k, v) in map.iter() {
                    params.push(self.write_graphson(&k.clone().into())?);
                    params.push(self.write_graphson(&v)?);
                }

                Ok(json!({
                    "@type" : "g:Map",
                    "@value" : params
                }))
            }
            (_, GValue::T(t)) => {
                let v = match t {
                    T::Id => "id",
                    T::Key => "key",
                    T::Label => "label",
                    T::Value => "value",
                };

                Ok(json!({
                    "@type" : "g:T",
                    "@value" : v
                }))
            }
            (_, GValue::Scope(s)) => {
                let v = match s {
                    Scope::Global => "global",
                    Scope::Local => "local",
                };

                Ok(json!({
                    "@type" : "g:Scope",
                    "@value" : v
                }))
            }

            (_, GValue::Order(s)) => {
                let v = match s {
                    Order::Asc => "asc",
                    Order::Desc => "desc",
                    Order::Shuffle => "shuffle",
                };

                Ok(json!({
                    "@type" : "g:Order",
                    "@value" : v
                }))
            }
            (_, GValue::Bool(b)) => {
                let json_string = match b {
                    true => "true",
                    false => "false",
                };
                Ok(serde_json::from_str(json_string).unwrap())
            }
            (_, GValue::TextP(text_p)) => Ok(json!({
                "@type" : "g:TextP",
                "@value" : {
                    "predicate" : text_p.operator(),
                    "value" : self.write_graphson(text_p.value())?
                }
            })),
            (_, GValue::Pop(pop)) => Ok(json!({
                "@type": "g:Pop",
                "@value": *pop.to_string(),
            })),
            (_, GValue::Cardinality(cardinality)) => {
                let v = match cardinality {
                    Cardinality::List => "list",
                    Cardinality::Single => "single",
                    Cardinality::Set => "set",
                };
                Ok(json!({
                    "@type" : "g:Cardinality",
                    "@value" : v
                }))
            }
            (_, GValue::Merge(merge)) => {
                let merge_option = match merge {
                    Merge::OnCreate => "onCreate",
                    Merge::OnMatch => "onMatch",
                    Merge::OutV => "outV",
                    Merge::InV => "inV",
                };
                Ok(json!({
                    "@type" : "g:Merge",
                    "@value" : merge_option
                }))
            }
            (_, GValue::Direction(direction)) => {
                let direction = match direction {
                    Direction::Out | Direction::From => "OUT",
                    Direction::In | Direction::To => "IN",
                };
                Ok(json!({
                    "@type" : "g:Direction",
                    "@value" : direction,
                }))
            }
            (_, GValue::Column(column)) => {
                let column = match column {
                    crate::structure::Column::Keys => "keys",
                    crate::structure::Column::Values => "values",
                };
                Ok(json!({
                    "@type" : "g:Column",
                    "@value" : column,
                }))
            }
            (_, _) => panic!("Type {:?} not supported.", value),
        }
    }

    pub fn content_type(&self) -> &str {
        match self {
            IoProtocol::GraphSONV2 => "application/vnd.gremlin-v2.0+json",
            IoProtocol::GraphSONV3 => "application/vnd.gremlin-v3.0+json",
            IoProtocol::GraphBinaryV1 => "application/vnd.graphbinary-v1.0",
        }
    }
}
