macro_rules! g_serializer {
    ($name:ident, { $($key:expr => $value:ident),*}) => {
        pub fn $name(val: &Value) -> GremlinResult<GValue> {
            if let Value::String(ref s) = val {
                Ok(s.clone().into())
            } else if let Value::Null = val {
                Ok(GValue::Null)
            } else if let Value::Bool(b) = val {
                Ok((*b).into())
            } else {
                let _type = &val["@type"];
                let _type = get_value!(_type,serde_json::Value::String)?.as_str();
                let _value = &val["@value"];

                match _type {
                    $($key => {
                        $value(&$name,_value)
                    })*
                    _ => Err($crate::GremlinError::Json(format!("Type {} not supported",_type)))
                }
            }
        }
    };
}

macro_rules! g_serializer_2 {
    ($name:ident, { $($key:expr => $value:ident),*}) => {
        pub fn $name(val: &Value) -> GremlinResult<GValue> {
            if let Value::String(ref s) = val {
                return Ok(s.clone().into())
            } else if let Value::Null = val {
                return Ok(GValue::Null)
            }
            if let Value::Array(_) = val {
                let _type = "g:List";
                let _value = &val;

                return match _type {
                    $($key => {
                        $value(&$name,_value)
                    })*
                    _ => Err($crate::GremlinError::Json(format!("Type {} not supported",_type)))
                }
            }
            if let Value::Object(ref o) = val {
                if o.contains_key("@type") {
                    let _type = o.get("@type").ok_or_else(|| $crate::GremlinError::Generic("Type missing".to_string()))?.as_str().ok_or_else(|| $crate::GremlinError::Generic("Type should be a string".to_string()))?;
                    let _value = &o.get("@value").ok_or_else(|| $crate::GremlinError::Generic("Value missing".to_string()))?;

                    return match _type {
                        $($key => {
                            $value(&$name,_value)
                        })*
                        _ => Err($crate::GremlinError::Json(format!("Type {} not supported",_type)))
                    }
                }
            }

            let _type = "g:Map";
            let _value = &val;

            match _type {
                $($key => {
                    $value(&$name,_value)
                })*
                _ => Err($crate::GremlinError::Json(format!("Type {} not supported",_type)))
            }
        }
    };
}

macro_rules! get_value {
    ($value:expr,$v:path) => {
        match $value {
            $v(e) => Ok(e),
            _ => Err($crate::GremlinError::Json(String::from(stringify!($v)))),
        }
    };
}

macro_rules! expect_i32 {
    ($value:expr) => {
        match $value.as_i64() {
            Some(v) => Ok(v),
            None => Err($crate::GremlinError::Json(String::from("Expected i32"))),
        }? as i32
    };
}

macro_rules! expect_i64 {
    ($value:expr) => {
        match $value.as_i64() {
            Some(v) => Ok(v),
            None => Err($crate::GremlinError::Json(String::from("Expected i64"))),
        }?
    };
}
macro_rules! expect_float {
    ($value:expr) => {
        match $value.as_f64() {
            Some(v) => Ok(v),
            None => Err($crate::GremlinError::Json(String::from("Expected float"))),
        }? as f32
    };
}
macro_rules! expect_double {
    ($value:expr) => {
        match $value.as_f64() {
            Some(v) => Ok(v),
            None => Err($crate::GremlinError::Json(String::from("Expected double"))),
        }?
    };
}
