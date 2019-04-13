#[macro_use]
mod macros;
mod serializer_v3;

use crate::structure::GValue;
use serde_json::{json, Value};

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
            _ => panic!("Type {:?} not supported.", value),
        }
    }
}
