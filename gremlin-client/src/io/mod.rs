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
            _ => unimplemented!(),
        }
    }
}
