#[macro_use]
mod macros;
mod serializer_v3;

use crate::conversion::ToGValue;
use crate::process::traversal::{Order, Scope};
use crate::structure::{GValue, T};
use serde_json::{json, Value};
use std::string::ToString;

use crate::GremlinResult;

#[derive(Debug, Clone)]
pub enum GraphSON {
    V3,
}

impl GraphSON {
    pub fn read(&self, value: &Value) -> GremlinResult<Option<GValue>> {
        if let Value::Null = value {
            return Ok(None);
        }
        match self {
            GraphSON::V3 => serializer_v3::deserializer_v3(value).map(Some),
        }
    }

    pub fn write(&self, value: &GValue) -> GremlinResult<Value> {
        match value {
            GValue::Double(f) => Ok(json!({
                "@type" : "g:Double",
                "@value" : f
            })),
            GValue::Float(f) => Ok(json!({
                "@type" : "g:Float",
                "@value" : f
            })),
            GValue::Int32(f) => Ok(json!({
                "@type" : "g:Int32",
                "@value" : f
            })),
            GValue::Int64(f) => Ok(json!({
                "@type" : "g:Int64",
                "@value" : f
            })),

            GValue::String(s) => Ok(Value::String(s.clone())),

            GValue::Uuid(s) => Ok(json!({
                "@type" : "g:UUID",
                "@value" : s.to_string()
            })),
            GValue::Date(d) => Ok(json!({
                "@type" : "g:Date",
                "@value" : d.timestamp()
            })),

            GValue::List(d) => {
                let elements: GremlinResult<Vec<Value>> = d.iter().map(|e| self.write(e)).collect();
                Ok(json!({
                    "@type" : "g:List",
                    "@value" : elements?
                }))
            }

            GValue::P(p) => Ok(json!({
                "@type" : "g:P",
                "@value" : {
                    "predicate" : p.operator(),
                    "value" : self.write(p.value())?
                }
            })),

            GValue::Bytecode(code) => {
                let steps: GremlinResult<Vec<Value>> = code
                    .steps()
                    .iter()
                    .map(|m| {
                        let mut instruction = vec![];
                        instruction.push(Value::String(m.operator().clone()));

                        let arguments: GremlinResult<Vec<Value>> =
                            m.args().iter().map(|a| self.write(a)).collect();

                        instruction.extend(arguments?);
                        Ok(Value::Array(instruction))
                    })
                    .collect();
                Ok(json!({
                    "@type" : "g:Bytecode",
                    "@value" : {
                        "step" : steps?
                    }
                }))
            }
            GValue::Vertex(v) => {
                let id = self.write(&v.id().to_gvalue())?;
                Ok(json!({
                    "@type" : "g:Vertex",
                    "@value" : {
                        "id" :  id,
                    }
                }))
            }
            GValue::Map(map) => {
                let mut params = vec![];

                for (k, v) in map.iter() {
                    params.push(self.write(&k.clone().into())?);
                    params.push(self.write(&v)?);
                }

                Ok(json!({
                    "@type" : "g:Map",
                    "@value" : params
                }))
            }
            GValue::T(t) => {
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
            GValue::Scope(s) => {
                let v = match s {
                    Scope::Global => "global",
                    Scope::Local => "local",
                };

                Ok(json!({
                    "@type" : "g:Scope",
                    "@value" : v
                }))
            }

            GValue::Order(s) => {
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

            GValue::Bool(b) => {
                let json_string = match b {
                    true => "true",
                    false => "false",
                };
                Ok(serde_json::from_str(json_string).unwrap())
            }

            GValue::TextP(text_p) => Ok(json!({
                "@type" : "g:TextP",
                "@value" : {
                    "predicate" : text_p.operator(),
                    "value" : self.write(text_p.value())?
                }
            })),

            GValue::Pop(pop) => Ok(json!({
                "@type": "g:Pop",
                "@value": *pop.to_string(),
            })),

            _ => panic!("Type {:?} not supported.", value),
        }
    }
}
