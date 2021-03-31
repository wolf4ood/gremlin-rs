#[macro_use]
mod macros;
mod serializer_v1;
mod serializer_v2;
mod serializer_v3;

use crate::conversion::ToGValue;
use crate::process::traversal::{Order, Scope};
use crate::structure::{Cardinality, GValue, T};
use serde_json::{json, Map, Value};
use std::string::ToString;

use crate::{GremlinError, GremlinResult};

#[derive(Debug, Clone)]
pub enum GraphSON {
    V1,
    V2,
    V3,
}

impl GraphSON {
    pub fn read(&self, value: &Value) -> GremlinResult<Option<GValue>> {
        if let Value::Null = value {
            return Ok(None);
        }
        match self {
            GraphSON::V1 => serializer_v1::deserializer_v1(value).map(Some),
            GraphSON::V2 => serializer_v2::deserializer_v2(value).map(Some),
            GraphSON::V3 => serializer_v3::deserializer_v3(value).map(Some),
        }
    }

    pub fn write(&self, value: &GValue) -> GremlinResult<Value> {
        match (self, value) {
            (GraphSON::V1, GValue::Double(d)) => Ok(json!(d)),
            (_, GValue::Double(d)) => Ok(json!({
                "@type" : "g:Double",
                "@value" : d
            })),
            (GraphSON::V1, GValue::Float(f)) => Ok(json!(f)),
            (_, GValue::Float(f)) => Ok(json!({
                "@type" : "g:Float",
                "@value" : f
            })),
            (GraphSON::V1, GValue::Int32(i)) => Ok(json!(i)),
            (_, GValue::Int32(i)) => Ok(json!({
                "@type" : "g:Int32",
                "@value" : i
            })),
            (GraphSON::V1, GValue::Int64(i)) => Ok(json!(i)),
            (_, GValue::Int64(i)) => Ok(json!({
                "@type" : "g:Int64",
                "@value" : i
            })),
            (_, GValue::String(s)) => Ok(Value::String(s.clone())),
            (GraphSON::V1, GValue::Uuid(u)) => Ok(json!(u)),
            (_, GValue::Uuid(s)) => Ok(json!({
                "@type" : "g:UUID",
                "@value" : s.to_string()
            })),
            (GraphSON::V1, GValue::Date(d)) => Ok(json!(d.timestamp_millis())),
            (_, GValue::Date(d)) => Ok(json!({
                "@type" : "g:Date",
                "@value" : d.timestamp_millis()
            })),
            (GraphSON::V1, GValue::List(d)) => {
                let elements: GremlinResult<Vec<Value>> = d.iter().map(|e| self.write(e)).collect();
                Ok(json!(elements?))
            }
            (GraphSON::V2, GValue::List(d)) => {
                let elements: GremlinResult<Vec<Value>> = d.iter().map(|e| self.write(e)).collect();
                Ok(json!(elements?))
            }
            (GraphSON::V3, GValue::List(d)) => {
                let elements: GremlinResult<Vec<Value>> = d.iter().map(|e| self.write(e)).collect();
                Ok(json!({
                    "@type" : "g:List",
                    "@value" : elements?
                }))
            }
            (_, GValue::P(p)) => Ok(json!({
                "@type" : "g:P",
                "@value" : {
                    "predicate" : p.operator(),
                    "value" : self.write(p.value())?
                }
            })),
            (GraphSON::V1, GValue::Bytecode(_code)) => {
                panic!("Bytecode not supported in GraphsonV1")
            }
            (_, GValue::Bytecode(code)) => {
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
            (_, GValue::Vertex(v)) => {
                let id = self.write(&v.id().to_gvalue())?;
                Ok(json!({
                    "@type" : "g:Vertex",
                    "@value" : {
                        "id" :  id,
                    }
                }))
            }
            (GraphSON::V1, GValue::Map(map)) => {
                let mut params = Map::new();

                for (k, v) in map.iter() {
                    params.insert(
                        self.write(&k.clone().into())?
                            .as_str()
                            .ok_or_else(|| {
                                GremlinError::Generic("Non-string key value.".to_string())
                            })?
                            .to_string(),
                        self.write(&v)?,
                    );
                }

                Ok(json!(params))
            }
            (GraphSON::V2, GValue::Map(map)) => {
                let mut params = Map::new();

                for (k, v) in map.iter() {
                    params.insert(
                        self.write(&k.clone().into())?
                            .as_str()
                            .ok_or_else(|| {
                                GremlinError::Generic("Non-string key value.".to_string())
                            })?
                            .to_string(),
                        self.write(&v)?,
                    );
                }

                Ok(json!(params))
            }
            (GraphSON::V3, GValue::Map(map)) => {
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
                    "value" : self.write(text_p.value())?
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

            (_, _) => panic!("Type {:?} not supported.", value),
        }
    }
}
