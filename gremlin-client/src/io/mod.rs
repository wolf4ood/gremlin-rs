#[macro_use]
mod macros;
mod serializer_v3;

use crate::structure::GValue;
use serde_json::Value;

use crate::{GremlinError, GremlinResult};

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
            GValue::Double(f) => Ok(Value::Number(
                serde_json::Number::from_f64(*f)
                    .ok_or_else(|| GremlinError::Json(String::from("Error serializing number")))?,
            )),
            GValue::Float(f) => Ok(Value::Number(
                serde_json::Number::from_f64((*f).into())
                    .ok_or_else(|| GremlinError::Json(String::from("Error serializing number")))?,
            )),
            GValue::Int32(f) => Ok(Value::Number(serde_json::Number::from(*f))),
            GValue::Int64(f) => Ok(Value::Number(serde_json::Number::from(*f))),
            GValue::String(s) => Ok(Value::String(s.clone())),

            GValue::Map(map) => {
                let v = map
                    .iter()
                    .map(|(k, v)| self.write(&v).map(|r| (k.clone(), r)))
                    .collect::<Result<serde_json::Map<String, Value>, GremlinError>>()?;
                Ok(Value::Object(v))
            }
            _ => unimplemented!(),
        }
    }
}
